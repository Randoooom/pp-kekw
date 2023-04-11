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
use crate::data::account::Account;
use crate::prelude::*;
use aide::axum::routing::put_with;
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::State;
use axum::Extension;

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            put_with(change_password, change_password_docs).layer(require_session!(state, DEFAULT)),
        )
        .with_state(state)
}

#[derive(Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    old_password: String,
    new_password: String,
    token: Option<String>,
}

async fn change_password(
    State(state): State<ApplicationState>,
    Extension(mut account): Extension<Account>,
    Json(data): Json<ChangePasswordRequest>,
) -> Result<Json<CreationResponse>> {
    let connection = state.connection();

    // try to change the password
    account
        .change_encryption_key(
            data.old_password.as_str(),
            data.new_password.as_str(),
            data.token.as_deref(),
            connection,
        )
        .await?;
    Ok(Json(CreationResponse::from(true)))
}

fn change_password_docs(op: TransformOperation) -> TransformOperation {
    op.description("change password")
        .response::<200, Json<CreationResponse>>()
        .security_requirement("Session")
}
