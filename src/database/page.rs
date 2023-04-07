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
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct Page<T>
where
    T: JsonSchema + Serialize,
{
    /// the result
    pub data: Vec<T>,
    /// the total count of pages
    pub pages: u64,
    /// the total count of elements
    pub total: u64,
    /// the offset for the next page
    pub next_page_offset: u64,
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PagingRequest {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    20
}

impl PagingRequest {
    #[instrument(skip_all)]
    pub async fn execute<T>(self, query: &str, connection: &DatabaseConnection) -> Result<Page<T>>
    where
        T: DeserializeOwned + JsonSchema + Serialize,
    {
        Page::<T>::select::<T>(
            query,
            self.page_size,
            (self.page - 1) * self.page_size,
            connection,
        )
        .await
    }
}

impl<T> Page<T>
where
    T: DeserializeOwned + JsonSchema + Serialize,
{
    pub async fn select<R>(
        query: &str,
        page_size: u64,
        offset: u64,
        connection: &DatabaseConnection,
    ) -> Result<Page<R>>
    where
        R: Serialize + DeserializeOwned + JsonSchema,
    {
        // build the count query
        let count = format!("SELECT * FROM count(({query}))");
        // build the paging query
        let query = format!("{query} LIMIT {page_size} START {offset}");

        // execute the queries
        let mut response = sql_span!(connection.query(count).query(query).await?.check()?);
        // get the total count of entries
        let total = response
            .take::<Option<u64>>(0)?
            .ok_or(ApplicationError::InternalServerError)?;
        // parse the entries
        let data = response.take::<Vec<R>>(1)?;

        Ok(Page {
            data,
            pages: (total as f64 / page_size as f64).ceil() as u64,
            total,
            next_page_offset: offset + page_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::database::page::Page;
    use axum::BoxError;

    #[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
    struct Test {
        foo: String,
    }

    #[tokio::test]
    async fn test_paging() -> Result<(), BoxError> {
        let connection = crate::database::connect().await?;
        let total = 25;

        let mut query = connection.query("LET $foo = 'bar'");
        for _ in 0..total {
            query = query.query("CREATE test SET foo = 'bar'");
        }
        query.await?.check()?;

        let page = Page::<Test>::select::<Test>("SELECT * FROM test", 10, 5, &connection).await?;
        assert_eq!(25u64, page.total);
        assert_eq!(15u64, page.next_page_offset);
        assert_eq!(3u64, page.pages);

        Ok(())
    }
}
