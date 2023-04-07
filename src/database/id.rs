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

use crate::error::ApplicationError;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::opt::{IntoResource, Resource};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    pub table: String,
    pub id: String,
}

impl TryFrom<(&str, &str)> for Id {
    type Error = ApplicationError;

    fn try_from((force, id): (&str, &str)) -> Result<Self, Self::Error> {
        let mut split = id.split(':');
        let table = split
            .next()
            .ok_or(ApplicationError::BadRequest("invalid id".to_owned()))?;
        // for security reasons we can't allow every table
        if !table.eq(force) {
            return Err(ApplicationError::Unauthorized);
        }

        let id = split
            .next()
            .ok_or(ApplicationError::BadRequest("invalid id".to_owned()))?;

        Ok(Self {
            table: table.to_string(),
            id: id.to_string(),
        })
    }
}

impl Id {
    pub fn new((table, id): (&str, &str)) -> Self {
        Self {
            table: table.to_string(),
            id: id.to_string(),
        }
    }

    pub fn to_thing(&self) -> Thing {
        Thing::from((self.table.as_str(), self.id.as_str()))
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value = serde_json::value::Value::deserialize(deserializer).unwrap();

        if let Some(string) = raw_value.as_str() {
            let mut split = string.split(':');
            let table = split
                .next()
                .ok_or(serde::de::Error::custom("Invalid id format"))?
                .to_string();
            let id = split
                .next()
                .ok_or(serde::de::Error::custom("Invalid id format"))?
                .to_string();

            return Ok(Self { table, id });
        }

        if raw_value.is_object() {
            // deserialize it as `Thing`
            // TODO: map err
            let thing = serde_json::from_value::<Thing>(raw_value).unwrap();
            return Ok(Self {
                table: thing.tb,
                id: thing.id.to_string(),
            });
        }

        Err(serde::de::Error::custom("Invalid datatype"))
    }
}

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("{}:{}", &self.table, &self.id)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl JsonSchema for Id {
    fn schema_name() -> String {
        "Id".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("string".to_string()),
            ..Default::default()
        }
        .into()
    }
}

impl<R> IntoResource<Option<R>> for &Id {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(Resource::RecordId(self.to_thing()))
    }
}
