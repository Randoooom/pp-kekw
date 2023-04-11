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
use chrono::{DateTime, Utc};

pub mod create;
pub mod protected;

#[derive(Getters, Deserialize, Serialize, Clone, Debug, Setters)]
#[cfg_attr(test, derive(PartialEq))]
#[getset(get = "pub")]
#[set = "pub"]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// primary and unique identifier
    pub id: Id,
    /// the username
    pub username: String,
    /// the uuid of the linked minecraft account
    pub uuid: Option<String>,
    /// the double hashed password
    pub password: String,
    /// the totp secret
    pub secret: String,
    /// the hashing nonce
    pub nonce: String,
    /// is totp enabled
    pub totp: bool,
    /// is locked
    pub locked: bool,
    #[serde(alias = "created_at")]
    pub created_at: DateTime<Utc>,
}

impl Account {
    /// Get an instance of an account by the given username. This operation will fail if the username
    /// cant be associated with an account.
    #[instrument(skip(connection))]
    pub async fn from_username(
        username: &str,
        connection: &DatabaseConnection,
    ) -> Result<Option<Self>> {
        // fetch the account from the database
        let account = sql_span!(connection
            .query("SELECT * FROM account WHERE username = $username")
            .bind(("username", username))
            .await?
            .take::<Option<Account>>(0)?);

        Ok(account)
    }

    /// Get an instance of an account by the given id. This operation will fail if the id
    /// cant be associated with an account.
    #[instrument(skip(connection))]
    pub async fn from_id(id: &str, connection: &DatabaseConnection) -> Result<Option<Self>> {
        // fetch the account from the database
        let account: Option<Account> =
            sql_span!(connection.select(&Id::try_from(("account", id))?).await?);

        Ok(account)
    }

    /// link the account with the given minecraft uuid.
    #[instrument(skip_all)]
    pub async fn link(&mut self, uuid: &str, connection: &DatabaseConnection) -> Result<()> {
        self.uuid = Some(uuid.to_string());
        // update in the database
        sql_span!(connection
            .query("UPDATE $account SET uuid = $uuid")
            .bind(("account", self.id().to_thing()))
            .bind(("uuid", uuid))
            .await?
            .check()?);

        Ok(())
    }
}
