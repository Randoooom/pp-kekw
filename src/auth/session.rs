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
use chrono::{Duration, Utc};

/// The length of an issued session in seconds.
/// Default is 3600(1 hour)
const SESSION_LENGTH: i64 = 3600;
/// The duration a refresh is authorized
const REFRESH_LENGTH: i64 = 5400;

/// The type of the issued session. Available are `Machine` which represents an API-Client or similar
/// things identified by their internal id. `Human` represents the id of the concerned `Account`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SessionType {
    Machine(String),
    Human(String),
}

impl ToString for SessionType {
    fn to_string(&self) -> String {
        match self {
            SessionType::Human(id) => id.clone(),
            SessionType::Machine(id) => id.clone(),
        }
    }
}

/// A session stores all the relevant information needed to authenticate and authorize incoming requests
/// by their given `sessionId`. All sessions can be refreshed
#[derive(Clone, Debug, Getters, Deserialize, Serialize)]
pub struct Session {
    id: String,
    #[get = "pub"]
    target: SessionType,
    #[get = "pub"]
    iat: i64,
    #[get = "pub"]
    exp: i64,
    refresh_token: String,
    refresh_exp: i64,
}

impl Session {
    /// Initializes a new session for the given target and saves it into the database.
    #[instrument(skip(connection))]
    pub async fn init(target: SessionType, connection: &DatabaseConnection) -> Result<Self> {
        // stop any other may existing sessions
        sql_span!(
            connection
                .query("DELETE FROM session WHERE target = $target")
                .bind(("target", target.to_string()))
                .await?,
            "end existing sessions"
        );

        let session = Self {
            id: nanoid::nanoid!(32),
            target,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::seconds(SESSION_LENGTH)).timestamp(),
            refresh_token: nanoid::nanoid!(64),
            refresh_exp: (Utc::now() + Duration::seconds(REFRESH_LENGTH)).timestamp(),
        };
        // save the session into the database
        sql_span!(
            connection
                .create(("session", session.id.clone()))
                .content(&session)
                .await?
        );

        Ok(session)
    }

    /// Check whether the given session is valid or not.
    #[instrument(skip(connection))]
    pub async fn is_session_valid(id: &str, connection: &DatabaseConnection) -> Result<()> {
        // try to fetch the session out of the database
        let session: Option<Session> = sql_span!(connection.select(("session", id)).await?);

        match session {
            Some(session) => Self::is_valid(&session, connection).await,
            None => Err(ApplicationError::Unauthorized),
        }
    }

    #[instrument(skip_all)]
    pub async fn is_valid(&self, connection: &DatabaseConnection) -> Result<()> {
        return if Utc::now().timestamp() >= self.exp {
            // the session is not anymore valid, so we end it.
            self.end(connection).await?;

            Err(ApplicationError::Unauthorized)
        } else {
            Ok(())
        };
    }

    /// Ends the given session
    #[instrument(skip_all)]
    pub async fn end(self, connection: &DatabaseConnection) -> Result<()> {
        sql_span!(connection.delete(("session", self.id)).await?);

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn refresh(
        &mut self,
        refresh_token: &str,
        connection: &DatabaseConnection,
    ) -> Result<()> {
        return if self.refresh_token.eq(refresh_token) {
            // change the iat and the exp
            self.iat = Utc::now().timestamp();
            self.exp = (Utc::now() + Duration::seconds(SESSION_LENGTH)).timestamp();
            // regenerate the refresh token
            self.refresh_token = nanoid::nanoid!(64);
            self.refresh_exp = (Utc::now() + Duration::seconds(REFRESH_LENGTH)).timestamp();

            // push the changes into the database
            sql_span!(
                connection
                    .update(("session", &self.id))
                    .content(&self)
                    .await?
            );

            Ok(())
        } else {
            self.end(connection).await?;

            Err(ApplicationError::Unauthorized)
        };
    }
}
