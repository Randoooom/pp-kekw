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

use crate::prelude::*;
use aide::axum::ApiRouter;

mod account;
mod auth;
pub mod docs;
mod event;
pub mod extractor;
mod middleware;
mod news;

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/auth", auth::router(state.clone()))
        .nest_api_service("/account", account::router(state.clone()))
        .nest_api_service("/event", event::router(state.clone()))
        .nest_api_service("/news", news::router(state.clone()))
        .with_state(state)
}

#[derive(Serialize, Debug, Clone, JsonSchema)]
pub struct DeletionResponse {
    pub deleted: bool,
}

impl From<bool> for DeletionResponse {
    fn from(value: bool) -> Self {
        Self { deleted: value }
    }
}

#[derive(Serialize, Debug, Clone, JsonSchema)]
pub struct CreationResponse {
    pub created: bool,
}

impl From<bool> for CreationResponse {
    fn from(value: bool) -> Self {
        Self { created: value }
    }
}
