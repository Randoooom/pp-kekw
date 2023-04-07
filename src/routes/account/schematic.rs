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

use crate::data::account::Account;
use crate::data::schematic::Schematic;
use crate::database::DatabaseResult;
use crate::prelude::*;
use aide::axum::routing::{get_with, post_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::body::StreamBody;
use axum::extract::{BodyStream, Path, State};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::Extension;
use futures::TryStreamExt;
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio_util::io::{ReaderStream, StreamReader};

pub fn router(state: ApplicationState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/upload/:schematic_name",
            post_with(upload, upload_docs).layer(require_session!(state, DEFAULT)),
        )
        .api_route(
            "/:schematic_id",
            get_with(download, download_docs)
                .delete_with(delete, delete_docs)
                .layer(require_session!(state, DEFAULT)),
        )
        .with_state(state)
}

/// POST /account/:account_id/schematic/:schematic_name
async fn upload(
    State(state): State<ApplicationState>,
    Path((_account_id, schematic_name)): Path<(String, String)>,
    Extension(account): Extension<Account>,
    body: BodyStream,
) -> Result<(StatusCode, Json<CreationResponse>)> {
    let connection = state.connection();

    // enforce a linked minecraft uuid
    match account.uuid() {
        Some(uuid) => {
            // read the async stream
            let mut data: Vec<u8> = Vec::new();
            let stream_reader = StreamReader::new(
                body.map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
            );
            futures::pin_mut!(stream_reader);
            stream_reader.read_to_end(&mut data).await?;
            // encode the data as bas64
            let encoded = openssl::base64::encode_block(data.as_slice());

            // save into the database
            sql_span!(
                connection
                    .create("schematic")
                    .content(&serde_json::json! ({
                        "owner": uuid,
                        "data": encoded,
                        "name": schematic_name
                    }))
                    .await?
            );

            Ok((StatusCode::CREATED, Json(CreationResponse::from(true))))
        }
        None => Err(ApplicationError::Unauthorized),
    }
}

fn upload_docs(op: TransformOperation) -> TransformOperation {
    op.description("Upload a new schematic")
        .response::<201, Json<CreationResponse>>()
        .response::<400, Json<ApplicationErrorResponse>>()
        .security_requirement("Session")
}

/// GET /account/:account_id/schematic/download/:schematic_id
async fn download(
    State(state): State<ApplicationState>,
    Path((_account_id, schematic_id)): Path<(String, String)>,
    Extension(account): Extension<Account>,
) -> Result<(HeaderMap, StreamBody<ReaderStream<Cursor<Vec<u8>>>>)> {
    let connection = state.connection();

    // fetch the schematic
    let schematic: Option<Schematic> = sql_span!(
        connection
            .select(&Id::try_from(("schematic", schematic_id.as_str()))?)
            .await?
    );
    match schematic {
        Some(schematic) => {
            // only allow the owner to download it
            if !schematic.owner().eq(&account
                .uuid()
                .clone()
                .ok_or(ApplicationError::Unauthorized)?)
            {
                return Err(ApplicationError::Unauthorized);
            }

            // decode the file
            let decoded = openssl::base64::decode_block(schematic.data().as_str())
                .map_err(|_| ApplicationError::InternalServerError)?;
            // turn into queue
            let cursor = Cursor::new(decoded);
            // read it as stream
            let stream = ReaderStream::new(cursor);
            // convert it to a body
            let body = StreamBody::new(stream);

            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str("text/schem; charset=utf-8").unwrap(),
            );
            headers.insert(
                CONTENT_DISPOSITION,
                HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap(),
            );

            Ok((headers, body))
        }
        None => Err(ApplicationError::BadRequest(
            "schematic not found".to_owned(),
        )),
    }
}

fn download_docs(op: TransformOperation) -> TransformOperation {
    op.description("download a schematic")
        .response::<401, Json<ApplicationErrorResponse>>()
        .security_requirement("Session")
}

pub async fn delete(
    State(state): State<ApplicationState>,
    Extension(account): Extension<Account>,
    Path((_account_id, schematic_id)): Path<(String, String)>,
) -> Result<Json<DeletionResponse>> {
    let connection = state.connection();

    // TODO: execute in single db query
    // fetch the owner of the schematic
    let id = Id::try_from(("schematic", schematic_id.as_str()))?;
    let owner = sql_span!(connection
        .query("SELECT owner as result FROM $schematic")
        .bind(("schematic", id.to_thing()))
        .await?
        .take::<Option<DatabaseResult<String>>>(0)?
        .ok_or(ApplicationError::Unauthorized)?);
    if owner.result.eq(&account
        .uuid()
        .clone()
        .ok_or(ApplicationError::Unauthorized)?)
    {
        sql_span!(connection.delete(&id).await?);

        Ok(Json(DeletionResponse::from(true)))
    } else {
        Err(ApplicationError::Unauthorized)
    }
}

fn delete_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a schematic")
        .response::<200, Json<DeletionResponse>>()
        .response::<401, Json<ApplicationErrorResponse>>()
}

// TODO: unittests
