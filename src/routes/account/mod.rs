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

use crate::data::account::create::CreateAccount;
use crate::data::account::protected::ProtectedAccount;
use crate::data::account::Account;
use crate::prelude::*;
use aide::axum::routing::{get_with, post_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::State;
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

#[cfg(test)]
mod tests {
    use crate::data::account::protected::ProtectedAccount;
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
}
