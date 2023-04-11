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

use crate::auth::session::Session;
use crate::auth::Authenticateable;
use crate::data::account::Account;
use crate::prelude::*;
use aide::axum::routing::post_with;
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Extension;
#[cfg(not(debug_assertions))]
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};

mod password;
mod totp;

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/login", post_with(login, login_docs))
        .api_route(
            "/logout",
            post_with(logout, logout_docs).layer(require_session!(state, DEFAULT)),
        )
        .api_route("/refresh", post_with(refresh, refresh_docs))
        .nest_api_service("/password", password::router(state.clone()))
        .nest_api_service("/totp", totp::router(state.clone()))
        .with_state(state)
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct LoginRequest {
    /// the username
    username: String,
    /// the password
    password: String,
    /// the totp token for optional enabled totp authentication
    token: Option<String>,
    #[cfg(not(debug_assertions))]
    hcaptcha: String,
}

/// POST /auth/login
async fn login(
    State(state): State<ApplicationState>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<Session>> {
    let connection = state.connection();

    // verify the hcaptcha token
    #[cfg(not(debug_assertions))]
    {
        let request = HcaptchaRequest::new(
            HCAPTCHA_SECRET.as_str(),
            HcaptchaCaptcha::new(data.hcaptcha.as_str())
                .map_err(|_| ApplicationError::Unauthorized)?,
        )
        .map_err(|_| ApplicationError::Unauthorized)?;
        let client = HcaptchaClient::new();

        client
            .verify_client_response(request)
            .await
            .map_err(|_| ApplicationError::Unauthorized)?;
    };
    // fetch the requested account
    match Account::from_username(data.username.as_str(), connection).await? {
        Some(account) => {
            // try to authorize the login
            account
                .login(data.password.as_str(), data.token.as_deref())
                .await?;

            // start a new session for the account
            let session = account.start_session(connection).await?;
            Ok(Json(session))
        }
        None => Err(ApplicationError::Unauthorized),
    }
}

fn login_docs(op: TransformOperation<'_>) -> TransformOperation {
    op.description("Start a new session")
        .response::<200, Json<Session>>()
        .response::<401, Json<ApplicationErrorResponse>>()
}

/// POST /auth/logout
async fn logout(
    Extension(session): Extension<Session>,
    State(state): State<ApplicationState>,
) -> Result<StatusCode> {
    let connection = state.connection();
    session.end(connection).await?;

    Ok(StatusCode::OK)
}

fn logout_docs(op: TransformOperation<'_>) -> TransformOperation {
    op.description("End an ongoing session")
        .response::<200, StatusCode>()
        .response::<401, Json<ApplicationErrorResponse>>()
        .security_requirement("Session")
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    // the session to refresh
    session_id: String,
    // the token
    refresh_token: String,
}

/// POST /auth/refresh
async fn refresh(
    State(state): State<ApplicationState>,
    Json(data): Json<RefreshRequest>,
) -> Result<Json<Session>> {
    let connection = state.connection();
    // fetch the session
    match Session::from_id(data.session_id.as_str(), connection).await? {
        Some(mut session) => {
            // try to refresh it
            session
                .refresh(data.refresh_token.as_str(), connection)
                .await?;

            Ok(Json(session))
        }
        None => Err(ApplicationError::Unauthorized),
    }
}

fn refresh_docs(op: TransformOperation) -> TransformOperation {
    op.description("Refresh the session")
        .response::<200, Json<Session>>()
        .response::<401, Json<ApplicationErrorResponse>>()
}
