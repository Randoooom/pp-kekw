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

#[derive(Getters, Deserialize, Serialize, Clone, Debug)]
#[getset(get = "pub")]
pub struct Account {
    /// primary and unique identifier
    pub id: String,
    /// display name
    pub username: String,
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
}
