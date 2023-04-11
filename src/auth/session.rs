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
use chrono::{Duration, Utc};

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// The length of an issued session in seconds.
/// Default is 3600(1 hour)
const SESSION_LENGTH: i64 = 3600;
/// The duration a refresh is authorized
const REFRESH_LENGTH: i64 = 5400;

/// The type of the issued session. Available are `Machine` which represents an API-Client or similar
/// things identified by their internal id. `Human` represents the id of the concerned `Account`
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type", content = "id")]
pub enum SessionType {
    Machine(Id),
    Human(Id),
}

impl ToString for SessionType {
    fn to_string(&self) -> String {
        match self {
            SessionType::Human(id) => id.to_string(),
            SessionType::Machine(id) => id.to_string(),
        }
    }
}

/// A session stores all the relevant information needed to authenticate and authorize incoming requests
/// by their given `sessionId`. All sessions can be refreshed
#[derive(Clone, Debug, Getters, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    /// the session id which will be used to authenticate the requests
    pub id: Id,
    /// the type of the session target. This can be a machine / api or an human user with it's Account.
    #[get = "pub"]
    target: SessionType,
    /// session started at timestamp (seconds)
    #[get = "pub"]
    iat: i64,
    /// session ends at timestamp (seconds)
    #[get = "pub"]
    exp: i64,
    /// token for rereshing the session on the `/auth/refresh` route
    #[serde(alias = "refresh_token")]
    refresh_token: String,
    /// refresh ends at timestamp (seconds)
    #[serde(alias = "refresh_exp")]
    refresh_exp: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateSession {
    target: SessionType,
    iat: i64,
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
                .await?
                .check()?,
            "end existing sessions"
        );

        let session = CreateSession {
            target,
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::seconds(SESSION_LENGTH)).timestamp(),
            refresh_token: nanoid::nanoid!(64),
            refresh_exp: (Utc::now() + Duration::seconds(REFRESH_LENGTH)).timestamp(),
        };
        // save the session into the database
        let session: Session = sql_span!(
            connection
                .create(("session", nanoid::nanoid!(32, &ALPHABET)))
                .content(&session)
                .await?
        );

        Ok(session)
    }

    /// Check whether the given session is valid or not.
    #[instrument(skip(connection))]
    pub async fn is_session_valid(id: &str, connection: &DatabaseConnection) -> Result<Session> {
        // try to fetch the session out of the database
        let session: Option<Session> =
            sql_span!(connection.select(&Id::try_from(("session", id))?).await?);

        match session {
            Some(session) => {
                if Self::is_valid(&session, connection).await.is_ok() {
                    Ok(session)
                } else {
                    Err(ApplicationError::Unauthorized)
                }
            }
            None => Err(ApplicationError::Unauthorized),
        }
    }

    #[instrument(skip_all)]
    pub async fn is_valid(&self, connection: &DatabaseConnection) -> Result<()> {
        if Utc::now().timestamp() >= self.exp {
            // the session is not anymore valid, so we end it.
            self.end(connection).await?;

            Err(ApplicationError::Unauthorized)
        } else {
            Ok(())
        }
    }

    /// Ends the given session
    #[instrument(skip_all)]
    pub async fn end(&self, connection: &DatabaseConnection) -> Result<()> {
        let _: Session = sql_span!(connection.delete(&self.id).await?);

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn refresh(
        &mut self,
        refresh_token: &str,
        connection: &DatabaseConnection,
    ) -> Result<()> {
        if self.refresh_token.eq(refresh_token) {
            // change the iat and the exp
            self.iat = Utc::now().timestamp();
            self.exp = (Utc::now() + Duration::seconds(SESSION_LENGTH)).timestamp();
            // regenerate the refresh token
            self.refresh_token = nanoid::nanoid!(64);
            self.refresh_exp = (Utc::now() + Duration::seconds(REFRESH_LENGTH)).timestamp();

            // push the changes into the database
            sql_span!(connection.update(&self.id).content(&self).await?);

            Ok(())
        } else {
            self.end(connection).await?;

            Err(ApplicationError::Unauthorized)
        }
    }

    /// Fetch a session by its id.
    pub async fn from_id(id: &str, connection: &DatabaseConnection) -> Result<Option<Self>> {
        let session: Option<Session> = connection.select(&Id::try_from(("session", id))?).await?;

        Ok(session)
    }
}
