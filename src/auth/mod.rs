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

use crate::auth::session::{Session, SessionType};
use crate::data::account::Account;
use crate::prelude::*;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chacha20poly1305::aead::{Aead, Key, OsRng};
use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305, XNonce};
use totp_rs::{Algorithm, Secret, TOTP};

pub mod authz;
pub mod session;

#[async_trait]
pub trait Authenticateable {
    async fn login(&self, password: &str, token: Option<&str>) -> Result<()>;

    async fn start_session(&self, connection: &DatabaseConnection) -> Result<Session>;

    async fn fetch_session(&self, connection: &DatabaseConnection) -> Result<Option<Session>>;

    async fn logout(&self, connection: &DatabaseConnection) -> Result<()>;

    fn regenerate_secret(&mut self, password: &str) -> Result<()>;

    fn read_secret(&self, password: &str) -> Result<String>;

    fn obtain_encryption_key(&self, password: &str) -> Result<[u8; 32]>;
}

/// Derives a (new) key from the given password using argon2id
#[instrument(skip_all)]
pub fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32]> {
    let mut target = [0u8; 32];
    Argon2::default().hash_password_into(
        password.as_bytes(),
        salt.as_str().as_bytes(),
        &mut target,
    )?;
    Ok(target)
}

/// Hashes the given key using argon2id
#[instrument(skip_all)]
pub fn hash_key(key: &[u8; 32]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(key, &salt)
        .unwrap()
        .to_string()
}

/// This encrypt the given data with the given key (argon2dwr hash) using xChaCha20Poly1305
#[instrument(skip_all)]
fn encrypt(key: &[u8; 32], data: &str) -> String {
    // setup the cipher
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(Key::<XChaCha20Poly1305>::from_slice(key));

    // encrypt the data
    let encrypted = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
    // encode
    format!(
        "{}:{}",
        openssl::base64::encode_block(nonce.as_slice()),
        openssl::base64::encode_block(encrypted.as_slice())
    )
}

/// This decrypts the given data with the given key (argon2d hash) using xChaCha20Poly1305
#[instrument(skip_all)]
fn decrypt(key: &[u8; 32], data: &str) -> String {
    // prepare the cipher
    let cipher = XChaCha20Poly1305::new(Key::<XChaCha20Poly1305>::from_slice(key));

    // read the encrypted data
    let mut split = data.split(':');
    let nonce = openssl::base64::decode_block(split.next().unwrap()).unwrap();
    let data = openssl::base64::decode_block(split.next().unwrap()).unwrap();

    // decrypt the secret
    let nonce = XNonce::from_slice(nonce.as_slice());
    let decrypted = cipher.decrypt(nonce, data.as_slice()).unwrap();

    // convert to the utf8 encoded base32
    String::from_utf8(decrypted).unwrap()
}

#[async_trait]
impl Authenticateable for Account {
    #[instrument(skip_all)]
    async fn login(&self, password: &str, token: Option<&str>) -> Result<()> {
        // derive the key
        let key = self.obtain_encryption_key(password)?;
        // compare the hashes
        Argon2::default().verify_password(&key, &PasswordHash::new(self.password.as_str())?)?;

        // check if totp is required
        if self.totp {
            return if let Some(token) = token {
                if TOTP::new(
                    Algorithm::SHA256,
                    6,
                    0,
                    30,
                    self.read_secret(password)?.as_bytes().to_vec(),
                    None,
                    "".to_owned(),
                )
                .unwrap()
                .check_current(token)?
                {
                    Ok(())
                } else {
                    Err(ApplicationError::Unauthorized)
                }
            } else {
                Err(ApplicationError::Forbidden("TOTP is required".to_owned()))
            };
        }

        Ok(())
    }

    #[instrument(skip_all)]
    async fn start_session(&self, connection: &DatabaseConnection) -> Result<Session> {
        Session::init(SessionType::Human(self.id.clone()), connection).await
    }

    #[instrument(skip_all)]
    async fn fetch_session(&self, connection: &DatabaseConnection) -> Result<Option<Session>> {
        // fetch the session
        let session = sql_span!(connection
            .query("SELECT * FROM session WHERE target = $target")
            .bind(("target", self.id()))
            .await?
            .take::<Option<Session>>(0)?);

        Ok(session)
    }

    #[instrument(skip_all)]
    async fn logout(&self, connection: &DatabaseConnection) -> Result<()> {
        match self.fetch_session(connection).await? {
            Some(session) => session.end(connection).await,
            None => Err(ApplicationError::Unauthorized),
        }
    }

    #[instrument(skip_all)]
    fn regenerate_secret(&mut self, password: &str) -> Result<()> {
        // generate a new secret
        let secret = Secret::generate_secret();
        // save the new secret
        self.secret = encrypt(
            &self.obtain_encryption_key(password)?,
            secret.to_string().as_str(),
        );

        Ok(())
    }

    #[instrument(skip_all)]
    fn read_secret(&self, password: &str) -> Result<String> {
        Ok(decrypt(
            &self.obtain_encryption_key(password)?,
            self.secret().as_str(),
        ))
    }

    #[instrument(skip_all)]
    fn obtain_encryption_key(&self, password: &str) -> Result<[u8; 32]> {
        derive_key(password, &SaltString::from_b64(self.nonce().as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::{decrypt, derive_key, encrypt, hash_key, Authenticateable};
    use crate::data::account::Account;
    use crate::prelude::Id;
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::SaltString;
    use chrono::{Duration, Local};
    use totp_rs::{Algorithm, TOTP};

    #[tokio::test]
    async fn test_login() {
        let password = "password";

        let salt = SaltString::generate(&mut OsRng);
        let hash = hash_key(&derive_key(password, &salt).unwrap());
        let account = Account {
            id: Id::new(("account", "")),
            username: "".to_owned(),
            uuid: None,
            password: hash,
            secret: "".to_string(),
            nonce: salt.to_string(),
            totp: false,
            locked: false,
            created_at: Default::default(),
        };

        assert!(account.login(password, None).await.is_ok());
        assert!(account.login("test", None).await.is_err());
    }

    #[tokio::test]
    async fn test_login_totp() {
        let password = "password";
        let salt = SaltString::generate(&mut OsRng);
        let hash = hash_key(&derive_key(password, &salt).unwrap());

        let mut account = Account {
            id: Id::new(("account", "")),
            username: "".to_owned(),
            uuid: None,
            password: hash,
            secret: "".to_string(),
            nonce: salt.to_string(),
            totp: true,
            locked: false,
            created_at: Default::default(),
        };
        account.regenerate_secret(password).unwrap();
        let totp = TOTP::new(
            Algorithm::SHA256,
            6,
            0,
            30,
            account.read_secret(password).unwrap().as_bytes().to_vec(),
            None,
            "".to_owned(),
        )
        .unwrap();

        assert!(account.login(password, None).await.is_err());
        assert!(account
            .login(
                password,
                Some(
                    totp.generate((Local::now() - Duration::seconds(300)).timestamp() as u64)
                        .as_str()
                )
            )
            .await
            .is_err());
        assert!(account
            .login(password, Some(totp.generate_current().unwrap().as_str()))
            .await
            .is_ok());
    }

    #[test]
    fn test_encryption() {
        let plaintext = "Hello, world!";
        let password = "password";

        let account = Account {
            id: Id::new(("account", "")),
            username: "".to_owned(),
            uuid: None,
            password: "".to_string(),
            secret: "".to_string(),
            nonce: SaltString::generate(&mut OsRng).to_string(),
            totp: false,
            locked: false,
            created_at: Default::default(),
        };
        let key = account.obtain_encryption_key(password).unwrap();
        let encrypted = encrypt(&key, plaintext);
        let decrypted = decrypt(&key, encrypted.as_str());

        assert_eq!(plaintext, decrypted);
    }
}
