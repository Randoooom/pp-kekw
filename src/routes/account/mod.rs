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

use crate::auth::authz::Authorizable;
use crate::data::account::create::CreateAccount;
use crate::data::account::protected::ProtectedAccount;
use crate::data::account::Account;
use crate::database::DatabaseResult;
use crate::prelude::*;
use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;

mod schematic;

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/signup", post_with(signup, signup_docs))
        .api_route(
            "/me",
            get_with(get_me, get_me_docs).layer(require_session!(state, DEFAULT)),
        )
        .api_route(
            "/:account_id",
            put_with(update_account_username, update_account_username_docs)
                .layer(require_session!(state, DEFAULT)),
        )
        .api_route(
            "/:account_id/permissions",
            get_with(get_permissions, get_permission_docs).layer(require_session!(state, DEFAULT)),
        )
        .nest_api_service("/:account_id/schematic", schematic::router(state.clone()))
        .with_state(state)
}

async fn signup(
    State(state): State<ApplicationState>,
    Json(data): Json<CreateAccount>,
) -> Result<(StatusCode, Json<CreationResponse>)> {
    let connection = state.connection();

    data.create(connection).await?;
    Ok((StatusCode::CREATED, Json(CreationResponse::from(true))))
}

fn signup_docs(op: TransformOperation) -> TransformOperation {
    op.description("SignUp a new account")
        .response::<201, Json<CreationResponse>>()
}

/// GET /account/me
async fn get_me(Extension(account): Extension<Account>) -> Result<Json<ProtectedAccount>> {
    Ok(Json(ProtectedAccount::from(account)))
}

fn get_me_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get authenticated account")
        .response::<200, Json<ProtectedAccount>>()
        .security_requirement("Session")
}

async fn get_permissions(
    State(state): State<ApplicationState>,
    Extension(requester): Extension<Account>,
    Path(account_id): Path<String>,
) -> Result<Json<Vec<Id>>> {
    let connection = state.connection();

    let target = if requester.id().to_string().eq(account_id.as_str()) {
        Ok(requester.id)
    } else {
        requester
            .has_permission(&ACCOUNT_PERMISSION_GET, connection)
            .await
            .and_then(|_| Id::try_from(("account", account_id.as_str())))
    }?;

    let permissions = sql_span!(connection
        .query("SELECT ->has->permission.id AS result FROM $target")
        .bind(("target", target.to_thing()))
        .await?
        .take::<Option<DatabaseResult<Vec<Id>>>>(0))?
    .ok_or(ApplicationError::InternalServerError)?;
    Ok(Json(permissions.result))
}

fn get_permission_docs(op: TransformOperation) -> TransformOperation {
    op.description("get permissions for the given account")
        .response::<200, Json<Vec<String>>>()
        .response::<401, Json<ApplicationErrorResponse>>()
        .description("Returns a list of the granted permissions for the target account. If the requester is \
        equal to the target the permissions will be returned. Otherwise the permission 'account.permission.get' is \
        be required to obtain a response.")
        .security_requirement("Session")
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct ChangeUsernameRequest {
    username: String,
}

async fn update_account_username(
    State(state): State<ApplicationState>,
    Extension(mut account): Extension<Account>,
    Path(id): Path<Id>,
    Json(data): Json<ChangeUsernameRequest>,
) -> Result<Json<ProtectedAccount>> {
    let connection = state.connection();

    if id.eq(account.id()) {
        account.username = data.username;
        // save into the database
        sql_span!(connection
            .query("UPDATE $account SET username = $username")
            .bind(("account", account.id().to_thing()))
            .bind(("username", account.username()))
            .await?
            .check()?);

        Ok(Json(ProtectedAccount::from(account)))
    } else {
        Err(ApplicationError::BadRequest(
            "not implemented yet".to_owned(),
        ))
    }
}

fn update_account_username_docs(op: TransformOperation) -> TransformOperation {
    op.description("change username")
        .response::<200, Json<ProtectedAccount>>()
        .security_requirement("Session")
}

#[cfg(test)]
mod tests {
    use crate::data::account::protected::ProtectedAccount;
    use crate::prelude::PERMISSIONS;
    use crate::tests::TestSuite;
    use axum::http::header::AUTHORIZATION;
    use axum::http::StatusCode;
    use axum::BoxError;

    #[tokio::test]
    async fn test_signup() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let response = suite
            .connector()
            .post("/account/signup")
            .json(&serde_json::json! ({
                "username": "test",
                "password": "password"
            }))
            .send()
            .await;
        assert_eq!(StatusCode::CREATED, response.status());
        assert!(suite.try_login("test", "password", None).await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_me() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = suite
            .connector()
            .get("/account/me")
            .header(AUTHORIZATION, session.as_str())
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let fetched = response.json::<ProtectedAccount>().await;
        assert_eq!(ProtectedAccount::from(suite.account().clone()), fetched);

        Ok(())
    }

    #[tokio::test]
    async fn test_change_username() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = suite
            .connector()
            .put(format!("/account/{}", suite.account().id().to_string()).as_str())
            .header(AUTHORIZATION, session.as_str())
            .json(&serde_json::json!({
                "username": "other"
            }))
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let account = response.json::<ProtectedAccount>().await;
        assert_eq!("other", account.username.as_str());
        assert!(suite.try_login("other", "password", None).await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_permissions() -> Result<(), BoxError> {
        let suite = TestSuite::start().await?;

        let session = suite.authenticate("username", "password", None).await;
        let response = suite
            .connector()
            .get(format!("/account/{}/permissions", suite.account().id().to_string()).as_str())
            .header(AUTHORIZATION, session.as_str())
            .send()
            .await;
        assert_eq!(StatusCode::OK, response.status());

        let permissions = response.json::<Vec<String>>().await;
        assert_eq!(PERMISSIONS.len(), permissions.len());

        Ok(())
    }
}
