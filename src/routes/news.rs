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

use crate::data::news::News;
use crate::prelude::*;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::{BodyStream, Path, Query, State};
use axum::http::StatusCode;
use futures::TryStreamExt;
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;

const CDN_DIR: &str = "cdn";

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create, create_docs).layer(require_session!(state, NEWS_CREATE)),
        )
        .api_route(
            "/:news_id",
            put_with(update_news, update_docs).layer(require_session!(state, NEWS_UPDATE)),
        )
        .api_route(
            "/:news_id/:file_extension",
            post_with(upload_image, upload_image_docs).layer(require_session!(state, NEWS_UPDATE)),
        )
        .api_route("/shown", get_with(get_shown, get_shown_docs))
        .api_route("/:news_id", get_with(get, get_docs))
        .api_route(
            "/",
            get_with(get_all, get_all_docs).layer(require_session!(state, NEWS_GET_ALL)),
        )
        .api_route(
            "/:news_id",
            delete_with(delete, delete_docs).layer(require_session!(state, NEWS_DELETE)),
        )
        .with_state(state)
}

#[derive(Deserialize, Debug, Clone, JsonSchema)]
pub struct CreateNewsRequest {
    /// the content of the news (html supported -> server side xss cleanup)
    content: String,
}

/// POST /news
async fn create(
    State(state): State<ApplicationState>,
    Json(data): Json<CreateNewsRequest>,
) -> Result<(StatusCode, Json<News>)> {
    let connection = state.connection();

    Ok((
        StatusCode::CREATED,
        Json(News::new(data.content.as_str(), connection).await?),
    ))
}

fn create_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create news")
        .response::<201, Json<News>>()
        .security_requirement_scopes("Session", vec![NEWS_CREATE.id.to_string()])
}

/// POST /news/:news_id/:file_extension
async fn upload_image(
    State(state): State<ApplicationState>,
    Path((news_id, file_extension)): Path<(String, String)>,
    body: BodyStream,
) -> Result<Json<CreationResponse>> {
    let connection = state.connection();

    // make sure the specified news do exist
    let news: Option<News> = sql_span!(
        connection
            .select(&Id::try_from(("news", news_id.as_str()))?)
            .await?
    );
    match news {
        Some(mut news) => {
            let stream_reader = StreamReader::new(
                body.map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
            );
            futures::pin_mut!(stream_reader);

            // create the file
            let path = std::path::Path::new(CDN_DIR).join("news");
            tokio::fs::create_dir_all(&path).await?;
            let path = path.join(news_id).with_extension(file_extension.as_str());
            let mut file = BufWriter::new(File::create(path).await?);
            // write the body stream to the file
            tokio::io::copy(&mut stream_reader, &mut file).await?;

            // save the new file extension
            news.set_extension(Some(file_extension));
            connection.update(news.id()).content(news).await?;

            Ok(Json(CreationResponse::from(true)))
        }
        None => Err(ApplicationError::BadRequest("news not found".to_string())),
    }
}

fn upload_image_docs(op: TransformOperation) -> TransformOperation {
    op.description("upload an thumbnail for a news object")
        .response::<200, Json<CreationResponse>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement_scopes("Session", vec![NEWS_UPDATE.id.to_string()])
}

/// GET /news
async fn get_all(
    State(state): State<ApplicationState>,
    Query(request): Query<PagingRequest>,
) -> Result<Json<Page<News>>> {
    let connection = state.connection();

    Ok(Json(
        request
            .execute::<News, &str>("SELECT * FROM news", None, connection)
            .await?,
    ))
}

fn get_all_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a page of news")
        .response::<200, Json<Page<News>>>()
        .security_requirement_scopes("Session", vec![NEWS_GET_ALL.id.to_string()])
}

/// GET /news/shown
async fn get_shown(State(state): State<ApplicationState>) -> Result<Json<Vec<News>>> {
    let connection = state.connection();

    let news = sql_span!(connection
        .query("SELECT * FROM news where shown = true")
        .await?
        .take::<Vec<News>>(0)?);
    Ok(Json(news))
}

fn get_shown_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get all shown / active news")
        .response::<200, Json<Vec<News>>>()
}

/// GET /news/:news_id
async fn get(
    State(state): State<ApplicationState>,
    Path(news_id): Path<String>,
) -> Result<Json<News>> {
    let connection = state.connection();

    let news: Option<News> = sql_span!(
        connection
            .select(&Id::try_from(("news", news_id.as_str()))?)
            .await?
    );
    Ok(Json(news.ok_or(ApplicationError::BadRequest(
        "news not found".to_owned(),
    ))?))
}

fn get_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a specific news object")
        .response::<200, Json<News>>()
        .response::<400, Json<ApplicationErrorResponse>>()
}

/// PUT /news/:news_id
async fn update_news(
    State(state): State<ApplicationState>,
    Path(news_id): Path<String>,
    Json(data): Json<News>,
) -> Result<Json<News>> {
    let connection = state.connection();

    if data.extension().is_none() && *data.shown() {
        return Err(ApplicationError::BadRequest(
            "extension has to be set in order to shown news".to_owned(),
        ));
    };

    let news: News = sql_span!(connection
        .query("UPDATE $news CONTENT $content")
        .bind(("news", Id::try_from(("news", news_id.as_str()))?.to_thing()))
        .bind(("content", data))
        .await?
        .take::<Option<News>>(0)?
        .ok_or(ApplicationError::BadRequest("news not found".to_owned()))?);

    Ok(Json(news))
}

fn update_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update already existing news")
        .response::<200, Json<News>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement_scopes("Session", vec![NEWS_UPDATE.id.to_string()])
}

/// DELETE /news/:news_id
async fn delete(
    State(state): State<ApplicationState>,
    Path(news_id): Path<String>,
) -> Result<Json<DeletionResponse>> {
    let connection = state.connection();

    sql_span!(
        connection
            .delete::<Option<serde_json::Value>>(&Id::try_from(("news", news_id.as_str()))?)
            .await?
    );
    Ok(Json(DeletionResponse::from(true)))
}

fn delete_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete the given news")
        .response::<200, Json<DeletionResponse>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement_scopes("Session", vec![NEWS_DELETE.id.to_string()])
}

#[cfg(test)]
mod tests {
    use crate::data::news::News;
    use crate::prelude::Page;
    use crate::tests::TestSuite;
    use axum::http::header::AUTHORIZATION;
    use axum::http::StatusCode;
    use axum::BoxError;
    use axum_test_helper::{TestClient, TestResponse};

    async fn setup(connector: &TestClient, session: &str) -> TestResponse {
        connector
            .post("/news")
            .header(AUTHORIZATION, session)
            .json(&serde_json::json!({
                "content": "nice content here",
            }))
            .send()
            .await
    }

    #[tokio::test]
    async fn test_create() -> Result<(), BoxError> {
        // TODO: test file upload
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
        let news = response.json::<News>().await;
        let response = suite
            .connector()
            .get(format!("/news/{}", news.id().to_string()).as_str())
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());
        let fetched = response.json::<News>().await;
        assert_eq!(news, fetched);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        setup(suite.connector(), session.as_str()).await;
        setup(suite.connector(), session.as_str()).await;

        let response = suite.connector().get("/news").send().await;
        assert_eq!(StatusCode::OK, response.status());
        let news = response.json::<Page<News>>().await;
        assert_eq!(2u64, news.total);
        assert_eq!(1u64, news.pages);
        assert_eq!(2, news.data.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        let news = response.json::<News>().await;

        let response = suite
            .connector()
            .delete(format!("/news/{}", news.id().to_string()).as_str())
            .header(AUTHORIZATION, session)
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let events: Vec<News> = suite.connection().select("news").await?;
        assert_eq!(0, events.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = setup(suite.connector(), session.as_str()).await;
        let news = response.json::<News>().await;

        let response = suite
            .connector()
            .put(format!("/news/{}", news.id().to_string()).as_str())
            .header(AUTHORIZATION, session)
            .json(&serde_json::json! ({
                "id": news.id().to_string(),
                "content": "other",
                "shown": true,
                "createdAt": news.created_at().clone()
            }))
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let fetched = response.json::<News>().await;
        assert_ne!(fetched, news);

        Ok(())
    }
}
