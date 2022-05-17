// Note: Both Rusqlite and MySQL Rust libs have native support for serde_json::Value.
// - Leaving this implementation here just in case another lib requires adding support (which means adding support for all of them).


use serde::{Serialize, Deserialize, Deserializer};

use crate::chrono::{NaiveDateTime};
use rusqlite::types::{ToSqlOutput, Value, FromSqlResult, ValueRef, FromSql};
use rusqlite::ToSql;
use mysql::prelude::{FromValue, ConvIr};
use std::any::type_name;
use mysql::FromValueError;
use mysql::chrono::{Datelike, Timelike, NaiveDate, NaiveTime};


use std::intrinsics::unreachable;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
// JSON = JS Object = JO
// - Using two letter notation to avoid conflict with `JSON`.
//      - This is mainly for use in Rust structs to encode the SQL table schema, so in the context of the Rust struct field name the type should make sense.
pub struct JO {
    pub jo: serde_json::Value,
}

impl From<JO> for serde_json::Value {
    fn from(x: JO) -> Self {
        x.jo
    }
}

impl From<serde_json::Value> for JO {
    fn from(x: serde_json::Value) -> Self {
        match x {
            Value::Object(_) | Value::Array(_) => {
                JO {
                    jo: x
                }
            }
            // Intended to be used to serialise to a SQL cell.
            // - SQL has its own primitives for scalar types - those should be used instead (`JSON.parse("null")` is a valid JSON object).
            _ => unreachable!("Converting from Value to JO should only be done for JS objects (object and array), never JSON scalar values")
        }
    }
}


impl ToSql for JO {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        Ok(ToSqlOutput::Owned(Value::Text(self.jo.to_string())))
    }
}


impl FromSql for JO {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let jo = match value {
            ValueRef::Text(x) => {
                use std::str;

                // Cases:
                // - A SQL null should be handled by the `Option` parent of this type, so this code is never reached.
                // - A JSON null is never stored (only objects).
                let jo = serde_json::from_str(str::from_utf8(x).unwrap()).unwrap();
                jo.into()
            }
            _ => unreachable!("Cannot convert non-SQLite-text col to DT3.")
        };

        Ok(jo)
    }
}

// @todo/low MySQL.