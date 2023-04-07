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

#[macro_export]
macro_rules! require_session {
    ($state:ident, $permission:path) => {{
        use axum::extract::State;
        use axum::http::header::AUTHORIZATION;
        use axum::http::Request;
        use axum::middleware::Next;
        use axum::response::{IntoResponse, Response};
        use $crate::auth::authz::Authorizable;
        use $crate::auth::session::{Session, SessionType};
        use $crate::data::account::Account;
        use $crate::prelude::*;

        async fn require_session<B>(
            State(state): State<ApplicationState>,
            mut request: Request<B>,
            next: Next<B>,
        ) -> Response {
            let response = match request.headers().get(AUTHORIZATION) {
                Some(value) => {
                    let span = info_span!("Authorizing the request");
                    let guard = span.enter();

                    // convert the header value to a string
                    let data = value.to_str().unwrap_or("");
                    // split the bearer off
                    let split = data.split(" ");
                    let token = split.last().unwrap_or("");

                    let connection = state.connection();
                    match Session::is_session_valid(token, connection).await {
                        Ok(session) => {
                            drop(guard);

                            let span = info_span!("Fetching the session target");
                            let guard = span.enter();
                            // borrow the mutable extensions
                            let extensions = request.extensions_mut();

                            if let SessionType::Human(id) = session.target() {
                                let account = Account::from_id(id.to_string().as_str(), connection)
                                    .await
                                    .unwrap()
                                    .unwrap();

                                if account
                                    .has_permission(&$permission, connection)
                                    .await
                                    .is_ok()
                                {
                                    extensions.insert(session);
                                    extensions.insert(account);
                                    Ok(())
                                } else {
                                    Err(ApplicationError::Unauthorized)
                                }
                            } else {
                                drop(guard);
                                Ok(())
                            }
                        }
                        Err(error) => Err(error),
                    }
                }
                None => Err(ApplicationError::Unauthorized),
            };

            match response {
                Ok(()) => next.run(request).await,
                Err(error) => error.into_response(),
            }
        }

        axum::middleware::from_fn_with_state($state.clone(), require_session)
    }};
}
