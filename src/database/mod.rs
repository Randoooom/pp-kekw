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

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{sql, Surreal};
use version_compare::{Cmp, Version};

const SURREALDB_ENDPOINT: &str = "SURREALDB_ENDPOINT";
const SURREALDB_USERNAME: &str = "SURREALDB_USERNAME";
const SURREALDB_PASSWORD: &str = "SURREALDB_PASSWORD";

pub type DatabaseConnection = Surreal<Client>;

pub async fn connect(up: &'static str) -> Result<DatabaseConnection> {
    // establish the connection
    let client: Surreal<Client> = Surreal::new::<Ws>(
        std::env::var(SURREALDB_ENDPOINT)
            .unwrap_or_else(|_| panic!("Missing {SURREALDB_ENDPOINT} env variable")),
    )
    .await?;
    info!("Established connection to surrealdb");

    // authenticate
    client
        .signin(Root {
            username: std::env::var(SURREALDB_USERNAME)
                .unwrap_or_else(|_| panic!("Missing {SURREALDB_USERNAME} env variable"))
                .as_str(),
            password: std::env::var(SURREALDB_PASSWORD)
                .unwrap_or_else(|_| panic!("Missing {SURREALDB_PASSWORD} env variable"))
                .as_str(),
        })
        .await?;
    tracing::info!("Authenticated with surrealdb");

    // use namespace and database
    cfg_if::cfg_if! {
        if #[cfg(feature = "testing")] {
            let db = nanoid::nanoid!();
            info!("Using database {db}");

            client
                .use_ns("test")
                .use_db(db)
                .await?;
        } else {
            client
                .use_ns("production")
                .use_db("template-service")
                .await?;
        }
    }

    // execute the up queries
    client.query(sql::parse(up)?).await?;
    info!("Initiated tables");

    // perform the migrations
    migrate(&client, env!("CARGO_PKG_VERSION"), Vec::new()).await?;

    Ok(client)
}

pub async fn migrate(
    client: &DatabaseConnection,
    current_version: &'static str,
    migrations: Vec<(&'static str, &'static str)>,
) -> Result<()> {
    // initiate the migration table and fetch possibly already existing records
    let mut responses = client
        .query(
            "DEFINE TABLE migration SCHEMALESS;
            DEFINE FIELD version     on TABLE migration TYPE string ASSERT $value IS NOT NULL;
            DEFINE FIELD created_at  on TABLE migration TYPE datetime VALUE time::now();",
        )
        .query(sql!(
            SELECT version, created_at FROM migration ORDER BY created_at DESC LIMIT 1
        ))
        .await?;
    // take the last as response, which contains the last migrated version
    let last = responses.take::<Option<String>>((1, "version"))?;

    if let Some(last) = last {
        // only proceed if the  last version is not equal to the current version
        if !last.as_str().eq(current_version) {
            // iterate through the given migrations
            for (version, migration) in migrations {
                if Version::from(last.as_str())
                    .unwrap()
                    .compare_to(Version::from(current_version).unwrap(), Cmp::Lt)
                {
                    info!("Executing surrealdb migration to {version}");
                    // execute the migration query and mark it as done
                    client
                        .query(migration)
                        .query(sql!(CREATE migration SET version = $version))
                        .bind(("version", version))
                        .await?;
                }
            }
        }
    } else {
        // insert the current version as the last version
        client
            .query(sql!(CREATE migration SET version = $version))
            .bind(("version", current_version))
            .await?;
    }

    Ok(())
}

#[macro_export]
macro_rules! sql_span {
    ($expr: expr) => {{
        let span = info_span!("Surrealdb Request");
        let _ = span.enter();
        $expr
    }};
    ($expr: expr, $title: expr) => {{
        let span = info_span!(concat!("Surrealdb Request: ", $title));
        let _ = span.enter();
        $expr
    }};
}
