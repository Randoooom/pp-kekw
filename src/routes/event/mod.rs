/*
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2023 Fritz Ochsmann
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

use crate::data::event::Event;
use crate::prelude::*;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create, create_docs).layer(require_session!(state, EVENT_CREATE)),
        )
        .api_route("/", get_with(get_all, get_all_docs))
        .api_route("/:event_id", get_with(get, get_docs))
        .api_route(
            "/:event_id",
            delete_with(delete, delete_docs).layer(require_session!(state, EVENT_DELETE)),
        )
        .api_route(
            "/:event_id",
            put_with(update, update_docs).layer(require_session!(state, EVENT_UPDATE)),
        )
        .with_state(state)
}

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct CreateEventRequest {
    /// the name / title for the planned event
    name: String,
    description: String,
    /// the scheduled start of the event
    start: DateTime<Utc>,
    /// the scheduled end of the event
    end: DateTime<Utc>,
}

/// POST /event
async fn create(
    State(state): State<ApplicationState>,
    Json(data): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<Event>)> {
    let connection = state.connection();

    // create the event
    let event: Event = sql_span!(connection.create("event").content(&data).await?);
    Ok((StatusCode::CREATED, Json(event)))
}

fn create_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new event")
        .response::<201, Json<Event>>()
        .security_requirement_scopes("Session", vec![EVENT_CREATE.id.to_string()])
}

/// GET /event/:event_id
async fn get(
    State(state): State<ApplicationState>,
    Path(event_id): Path<String>,
) -> Result<Json<Event>> {
    let connection = state.connection();

    let event: Option<Event> = connection
        .select(&Id::try_from(("event", event_id.as_str()))?)
        .await?;
    Ok(Json(event.ok_or(ApplicationError::BadRequest(
        "event not found".to_owned(),
    ))?))
}

fn get_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get the specified event")
        .response::<200, Json<Event>>()
        .response::<400, Json<ApplicationErrorResponse>>()
}

/// GET /event
async fn get_all<'a>(
    State(state): State<ApplicationState>,
    Query(data): Query<PagingRequest>,
) -> Result<Json<Page<Event>>> {
    let connection = state.connection();

    Ok(Json(
        data.execute::<Event, &str>("SELECT * FROM event", None, connection)
            .await?,
    ))
}

fn get_all_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a page of events")
        .response::<200, Json<Page<Event>>>()
}

/// DELETE /event/:event_id
async fn delete(
    State(state): State<ApplicationState>,
    Path(event_id): Path<String>,
) -> Result<Json<DeletionResponse>> {
    let connection = state.connection();

    sql_span!(
        connection
            .delete::<Option<serde_json::Value>>(&Id::try_from(("event", event_id.as_str()))?)
            .await?
    );
    Ok(Json(DeletionResponse::from(true)))
}

fn delete_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete the given event")
        .response::<200, Json<DeletionResponse>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement_scopes("Session", vec![EVENT_DELETE.id.to_string()])
}

/// PUT /event/:event_id
async fn update(
    State(state): State<ApplicationState>,
    Path(event_id): Path<String>,
    Json(data): Json<Event>,
) -> Result<Json<Event>> {
    let connection = state.connection();

    let event: Event = sql_span!(connection
        .query("UPDATE $event CONTENT $content")
        .bind((
            "event",
            Id::try_from(("event", event_id.as_str()))?.to_thing()
        ))
        .bind(("content", data))
        .await?
        .take::<Option<Event>>(0)?
        .ok_or(ApplicationError::BadRequest("event not found".to_owned()))?);
    Ok(Json(event))
}

fn update_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update an already existing event")
        .response::<200, Json<Event>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement_scopes("Session", vec![EVENT_UPDATE.id.to_string()])
}

#[cfg(test)]
mod tests {
    use crate::data::event::Event;
    use crate::prelude::Page;
    use crate::tests::TestSuite;
    use axum::http::header::AUTHORIZATION;
    use axum::http::StatusCode;
    use axum::BoxError;
    use axum_test_helper::{TestClient, TestResponse};
    use chrono::Utc;

    async fn setup(connector: &TestClient, session: &str) -> TestResponse {
        connector
            .post("/event")
            .header(AUTHORIZATION, session)
            .json(&serde_json::json!({
                "name": "name",
                "description": "description",
                "start": Utc::now(),
                "end": Utc::now(),
            }))
            .send()
            .await
    }

    #[tokio::test]
    async fn test_create() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        assert_eq!(StatusCode::CREATED, response.status());

        Ok(())
    }

    #[tokio::test]
    async fn test_get() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        let event = response.json::<Event>().await;
        let response = suite
            .connector()
            .get(format!("/event/{}", event.id().to_string()).as_str())
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());
        let fetched = response.json::<Event>().await;
        assert_eq!(event, fetched);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        setup(suite.connector(), session.as_str()).await;
        setup(suite.connector(), session.as_str()).await;

        let response = suite.connector().get("/event").send().await;
        let events = response.json::<Page<Event>>().await;
        assert_eq!(2u64, events.total);
        assert_eq!(1u64, events.pages);
        assert_eq!(2, events.data.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        let event = response.json::<Event>().await;

        let response = suite
            .connector()
            .delete(format!("/event/{}", event.id().to_string()).as_str())
            .header(AUTHORIZATION, session)
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let events: Vec<Event> = suite.connection().select("event").await?;
        assert_eq!(0, events.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        let event = response.json::<Event>().await;

        let response = suite
            .connector()
            .put(format!("/event/{}", event.id().to_string()).as_str())
            .header(AUTHORIZATION, session)
            .json(&serde_json::json! ({
                "id": event.id().to_string(),
                "name": "other",
                "description": "description",
                "start": event.start().clone(),
                "end": event.end().clone(),
                "createdAt": event.created_at().clone()
            }))
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let fetched = response.json::<Event>().await;
        assert_ne!(fetched, event);

        Ok(())
    }
}
