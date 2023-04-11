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

use crate::auth::Authenticateable;
use crate::data::account::protected::ProtectedAccount;
use crate::data::account::Account;
use crate::prelude::*;
use aide::axum::routing::put_with;
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::State;
use axum::Extension;
use totp_rs::{Algorithm, TOTP};

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            put_with(toggle, toggle_docs)
                .post_with(get_qr, get_qr_docs)
                .layer(require_session!(state, DEFAULT)),
        )
        .with_state(state)
}

#[derive(Serialize, JsonSchema, Debug, Clone)]
pub struct QrCodeResponse {
    // base64 encoded png
    data: String,
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct QrCodeRequest {
    password: String,
}

async fn get_qr(
    Extension(account): Extension<Account>,
    Json(data): Json<QrCodeRequest>,
) -> Result<Json<QrCodeResponse>> {
    // try to decode the secret
    let secret = account.read_secret(data.password.as_str())?;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.as_bytes().to_vec(),
        Some("MyPlayPlanet".to_owned()),
        account.username.clone(),
    )
    .map_err(|_| ApplicationError::Unauthorized)?;

    Ok(Json(QrCodeResponse {
        data: totp.get_qr().map_err(|_| ApplicationError::Unauthorized)?,
    }))
}

fn get_qr_docs(op: TransformOperation) -> TransformOperation {
    op.description("get the totp qr code")
        .response::<200, Json<QrCodeResponse>>()
        .security_requirement("Session")
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct TotpToggleRequest {
    password: String,
    token: String,
}

async fn toggle(
    State(state): State<ApplicationState>,
    Extension(mut account): Extension<Account>,
    Json(data): Json<TotpToggleRequest>,
) -> Result<Json<ProtectedAccount>> {
    let connection = state.connection();
    let new_state = !account.totp;

    // verify the request with forced totp activation
    account.set_totp(true);
    account
        .login(data.password.as_str(), Some(data.token.as_str()))
        .await?;

    // set the new state
    account.set_totp(new_state);
    // save into the database
    let _: Account = sql_span!(connection.update(account.id()).content(&account).await?);
    Ok(Json(ProtectedAccount::from(account)))
}

fn toggle_docs(op: TransformOperation) -> TransformOperation {
    op.description("Toggle the 2fa")
        .response::<200, Json<ProtectedAccount>>()
        .response::<401, Json<ApplicationErrorResponse>>()
        .security_requirement("Session")
}
