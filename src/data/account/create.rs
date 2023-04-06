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
use crate::prelude::*;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use totp_rs::Secret;

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CreateAccount {
    pub username: String,
    pub password: String,
}

impl CreateAccount {
    /// Create a new account with the given data. For security reasons the instance will be completely
    /// consumed by this function call.
    pub async fn create(self, connection: &DatabaseConnection) -> Result<Account> {
        // derive the key from the password
        let salt = SaltString::generate(&mut OsRng);
        let key = crate::auth::derive_key(self.password.as_str(), &salt)?;
        // hash the key
        let hash = crate::auth::hash_key(&key);

        // generate a secret
        let secret = Secret::generate_secret().to_string();

        // create the new account
        let account: Account = connection
            .create("account")
            .content(&serde_json::json! ({
                "username": self.username,
                "password": hash,
                "nonce": salt.to_string(),
                "secret": secret
            }))
            .await?;
        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::account::create::CreateAccount;
    use crate::data::account::Account;
    use axum::BoxError;

    #[tokio::test]
    async fn test_account_create() -> Result<(), BoxError> {
        let connection = &crate::database::connect().await?;

        let create = CreateAccount {
            username: "username".to_owned(),
            password: "password".to_owned(),
        };
        let account = create.create(connection).await?;

        let fetched = connection
            .query("SELECT * FROM $account")
            .bind(("account", account.id()))
            .await?
            .take::<Option<Account>>(0)?
            .unwrap();
        assert_eq!(account, fetched);

        Ok(())
    }
}
