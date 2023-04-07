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
use crate::auth::session::Session;
use crate::data::account::create::CreateAccount;
use crate::data::account::Account;
use crate::database::DatabaseConnection;
use crate::error::ApplicationError;
use crate::prelude::PERMISSIONS;
use axum::http::StatusCode;
use axum::BoxError;
use axum_test_helper::{TestClient, TestResponse};

#[derive(Getters)]
#[get = "pub"]
pub struct TestSuite {
    connector: TestClient,
    connection: DatabaseConnection,
    account: Account,
}

impl TestSuite {
    async fn create_account(connection: &DatabaseConnection) -> Result<Account, BoxError> {
        let data = CreateAccount {
            username: "username".to_owned(),
            password: "password".to_owned(),
        };
        let account = data.create(connection).await?;

        // grant the account all the available permissions
        for permission in PERMISSIONS.iter() {
            account.grant_permission(permission, connection).await?;
        }

        Ok(account)
    }

    async fn start_axum(connection: DatabaseConnection) -> Result<TestClient, BoxError> {
        Ok(TestClient::new(crate::router(connection).await?))
    }

    pub async fn start() -> Result<Self, BoxError> {
        let connection = crate::database::connect().await?;
        let connector = Self::start_axum(connection.clone()).await?;
        let account = Self::create_account(&connection).await?;

        Ok(Self {
            connector,
            connection,
            account,
        })
    }

    async fn login(&self, username: &str, password: &str, token: Option<&str>) -> TestResponse {
        self.connector
            .post("/auth/login")
            .json(&serde_json::json!({
                "username": username,
                "password": password,
                "token": token
            }))
            .send()
            .await
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
        token: Option<&str>,
    ) -> String {
        let response = self.login(username, password, token).await;
        let session = response.json::<Session>().await;

        session.id.to_string()
    }

    pub async fn try_login(
        &self,
        username: &str,
        password: &str,
        token: Option<&str>,
    ) -> Result<(), BoxError> {
        let response = self.login(username, password, token).await;

        match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(ApplicationError::Unauthorized)?,
        }
    }
}
