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
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub id: Id,
}

impl Permission {
    pub fn id(&self) -> String {
        self.id.to_string().replace('⟨', "").replace('⟩', "")
    }

    pub fn to_thing(&self) -> Thing {
        Thing::from(("permission", self.id.id.to_string().as_str()))
    }
}

macro_rules! permissions {
    ($(($ident:ident, $name:expr)),*) => {
        lazy_static::lazy_static! {
            $(
                pub static ref $ident: Permission = {
                    Permission {
                        id: Id::new(("permission", $name))
                    }
                };
            )*

            pub static ref DEFAULT: Permission = Permission {
                                                id: Id::new(("permission", "none")),
                                            };

            pub static ref PERMISSIONS: Vec<&'static Permission> = {
                vec![
                        $(
                            $ident.deref(),
                        )*
                    ]
            };
        }
    };
}

permissions!(
    (NEWS_CREATE, "news.create"),
    (NEWS_UPDATE, "news.update"),
    (NEWS_DELETE, "news.delete"),
    (NEWS_GET_ALL, "news.get.all"),
    // --------------------------------
    (EVENT_CREATE, "event.create"),
    (EVENT_UPDATE, "event.update"),
    (EVENT_DELETE, "event.delete"),
    // --------------------------------
    (EVENT_GROUP_CREATE, "event.group.create"),
    (EVENT_GROUP_UPDATE, "event.group.update"),
    (EVENT_GROUP_DELETE, "event.group.delete"),
    // --------------------------------
    (EVENT_FIGHT_CREATE, "event.fight.create"),
    (EVENT_FIGHT_UPDATE, "event.fight.update"),
    (EVENT_FIGHT_DELETE, "event.fight.delete"),
    // --------------------------------
    (ACCOUNT_PERMISSION_GET, "account.permission.get")
);

pub async fn init_permissions(connection: &DatabaseConnection) -> Result<()> {
    // fetch all currently available permissions
    let permissions: Vec<Permission> = connection.select("permission").await?;

    let mut query = String::new();
    PERMISSIONS
        .iter()
        .filter(|permission| !permissions.iter().any(|p| p.id().eq(&permission.id())))
        .for_each(|permission| {
            query.push_str(
                format!("CREATE type::thing('permission', '{}');", &permission.id.id).as_str(),
            )
        });
    if !query.is_empty() {
        // execute the query
        connection.query(query.as_str()).await?.check()?;
    }

    Ok(())
}
