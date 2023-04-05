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
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("{0}")]
    Forbidden(&'static str),
    #[error(transparent)]
    HashError(#[from] argon2::password_hash::errors::Error),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    SurrealdbError(#[from] surrealdb::Error),
    #[error("Internal error occurred")]
    InternalServerError,
}

impl From<argon2::Error> for ApplicationError {
    fn from(_: argon2::Error) -> Self {
        Self::Unauthorized
    }
}

pub type Result<T> = std::result::Result<T, ApplicationError>;

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            ApplicationError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            ),
            ApplicationError::BadRequest(error) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": error })))
            }
            ApplicationError::Forbidden(error) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": error })))
            }
            _ => {
                error!("{}", self.to_string());

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Error occurred while processing the request"})),
                )
            }
        }
        .into_response()
    }
}
