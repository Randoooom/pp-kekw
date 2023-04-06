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

use crate::auth::authz::permission::Permission;
use crate::data::account::Account;
use crate::database::DatabaseResult;
use crate::prelude::*;

pub mod permission;

#[async_trait]
pub trait Authorizable {
    async fn has_permission(
        &self,
        permission: &Permission,
        connection: &DatabaseConnection,
    ) -> Result<()>;

    async fn grant_permission(
        &self,
        permission: &Permission,
        connection: &DatabaseConnection,
    ) -> Result<()>;
}

#[async_trait]
impl Authorizable for Account {
    #[instrument(skip_all)]
    async fn has_permission(
        &self,
        permission: &Permission,
        connection: &DatabaseConnection,
    ) -> Result<()> {
        if permission.eq(&&DEFAULT) {
            return Ok(());
        }

        // query in the database
        let result = sql_span!(
            connection
                .query("select $permission INSIDE ->has->permission.id as result from $account",)
                .bind(("permission", &permission.id))
                .bind(("account", self.id()))
                .await?
        )
        .take::<Option<DatabaseResult<bool>>>(0)?
        .ok_or(ApplicationError::InternalServerError)?;

        result.is_ok()
    }

    #[instrument(skip_all)]
    async fn grant_permission(
        &self,
        permission: &Permission,
        connection: &DatabaseConnection,
    ) -> Result<()> {
        sql_span!(
            connection
                .query("RELATE $account->has->$permission")
                .bind(("account", self.id()))
                .bind(("permission", &permission.id))
                .await?
        );

        Ok(())
    }
}
