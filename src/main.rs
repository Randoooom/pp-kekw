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

#[macro_use]
extern crate serde;
#[macro_use]
extern crate schemars;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate aide;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate axum_macros;

use crate::prelude::{ApplicationState, DatabaseConnection};
use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use axum::http::{header, Method};
use axum::routing::get_service;
use axum::{BoxError, Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod auth;
mod data;
mod database;
mod error;
mod routes;
mod state;

#[cfg(test)]
mod tests;

pub async fn router(connection: DatabaseConnection) -> Result<Router, BoxError> {
    // connect to the database
    let state = ApplicationState::from(connection);

    aide::gen::extract_schemas(true);
    let cdn = ServeDir::new(".");
    let mut api = OpenApi::default();

    Ok(ApiRouter::new()
        .nest_api_service("/docs", routes::docs::router(state.clone()))
        .nest_api_service("/", routes::router(state))
        .finish_api_with(&mut api, routes::docs::transform_api)
        .route("/cdn/*name", get_service(cdn))
        .layer(
            CorsLayer::new()
                .allow_origin([std::env::var("ROOT").unwrap().parse().unwrap()])
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::HEAD,
                    Method::OPTIONS,
                ])
                .allow_headers(vec![
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::CONTENT_DISPOSITION,
                ])
                .expose_headers(vec![header::CONTENT_DISPOSITION]),
        )
        .layer(Extension(Arc::new(api))))
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let _ = std::env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET NOT FOUND");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection = database::connect().await?;
    let router = router(connection).await?;

    // start the axum server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub mod prelude {
    pub use crate::auth::authz::permission::*;
    pub use crate::database::id::Id;
    pub use crate::database::page::{Page, PagingRequest};
    pub use crate::database::DatabaseConnection;
    pub use crate::error::*;
    pub use crate::routes::extractor::Json;
    pub use crate::routes::{CreationResponse, DeletionResponse};
    pub use crate::state::ApplicationState;
    pub use crate::{require_session, sql_span};

    lazy_static::lazy_static! {
        pub static ref HCAPTCHA_SECRET: String = std::env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET NOT FOUND");
    }
}
