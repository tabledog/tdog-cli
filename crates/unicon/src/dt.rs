// Issue: `error[E0117]: only traits defined in the current crate can be implemented for arbitrary types`.
// Temp fix: Write to a string, eventually add either a field attr "transfrom_unix_to_iso", or "col_type=datetime, field: String".
// impl ToSql for DateTime<Utc> {
//     fn to_sql(&self) -> Result<ToSqlOutput<'_>, Error> {
//         let isoDateTime = self.format("%Y-%m-%d %H:%M:%S").to_string();
//         Ok(ToSqlOutput::Owned(Value::Text(isoDateTime)))
//     }
// }

use serde::{Serialize, Deserialize, Deserializer};

use crate::chrono::{Utc, DateTime, NaiveDateTime, TimeZone};
use rusqlite::types::{ToSqlOutput, Value, FromSqlResult, ValueRef, FromSql};
use rusqlite::ToSql;
use mysql::prelude::{FromValue, ConvIr};
use std::any::type_name;
use mysql::FromValueError;


use mysql::chrono::{Datelike, Timelike, NaiveDate, NaiveTime};
use postgres::types::{Type, IsNull, Kind};
use postgres::types::private::BytesMut;
use std::error::Error;

// Wrap to enable implementing traits for (sql_engine_x_trait, NaiveDateTime).
// Type name is DT{X} as from/to traits never pass metadata (so meta data must be passed in the type name, like a 3-digit-ms resolution).
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct DT {
    pub dt: NaiveDateTime,
}

// Enable parent type to have default derived, even though this value may always be set on struct instantiation.
impl Default for DT {
    fn default() -> Self {
        DT { dt: NaiveDateTime::from_timestamp(0, 0) }
    }
}


impl From<NaiveDateTime> for DT {
    fn from(dt: NaiveDateTime) -> Self {
        DT { dt }
    }
}

impl From<DT> for NaiveDateTime {
    fn from(x: DT) -> Self {
        x.dt
    }
}


impl From<DateTime<Utc>> for DT {
    fn from(x: DateTime<Utc>) -> Self {
        DT {
            dt: x.naive_utc()
        }
    }
}

impl From<DT> for DateTime<Utc> {
    fn from(x: DT) -> Self {
        Utc.from_utc_datetime(&x.dt)
    }
}


// Note: `rusqlite` has a feature `chrono` that implements this.
// - But it does not enforce a certain resolution?
//      - Or: It is unlikely that by default 3+ SQL engines will implement the to/from formatting to ms resolution by default (and/or provide options to give a custom resolution).
// @see https://github.com/rusqlite/rusqlite/blob/master/src/types/chrono.rs
impl ToSql for DT {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        let iso_string = self.dt.format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(ToSqlOutput::Owned(Value::Text(iso_string)))
    }
}


impl FromSql for DT {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let dt = match value {
            ValueRef::Text(x) => {
                use std::str;
                let dt = NaiveDateTime::parse_from_str(str::from_utf8(x).unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();
                DT { dt }
            }
            _ => unreachable!("Cannot convert non-SQLite-text col to DT.")
        };

        Ok(dt)
    }
}

use postgres::types::to_sql_checked;
impl postgres::types::ToSql for DT {

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
        self.dt.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        NaiveDateTime::accepts(ty)
    }

    // Used internally by crate rust-postgres.
    to_sql_checked!();
}


impl postgres::types::FromSql<'_> for DT {
    fn from_sql(ty: &Type, raw: &'_ [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let dt = NaiveDateTime::from_sql(ty, raw)?;
        Ok(DT { dt })
    }

    fn accepts(ty: &Type) -> bool {
        NaiveDateTime::accepts(ty)
    }
}


impl From<DT> for mysql::Value {
    fn from(dt: DT) -> Self {
        // year, month, day, hour, minutes, seconds, micro seconds

        let (year, month, day, hour, min, sec, micro) = (
            dt.dt.year() as u16,
            dt.dt.month() as u8,
            dt.dt.day() as u8,
            dt.dt.hour() as u8,
            dt.dt.minute() as u8,
            dt.dt.second() as u8,
            dt.dt.timestamp_subsec_micros() as u32,
        );

        let iso_string = dt.dt.format("%Y-%m-%d %H:%M:%S").to_string();
        mysql::Value::Date(year, month, day, hour, min, sec, micro)
    }
}


impl From<mysql::Value> for DT {
    fn from(x: mysql::Value) -> Self {
        match x {
            mysql::Value::Date(year, month, day, hour, min, sec, micro) => {
                DT {
                    dt: NaiveDateTime::new(
                        NaiveDate::from_ymd(year as i32, month as u32, day as u32),
                        NaiveTime::from_hms_micro(hour as u32, min as u32, sec as u32, micro),
                    )
                }
            }
            _ => unreachable!("Expected value from MySQL to be a Timestamp, found {:?}", &x)
        }
    }
}

// Handles null coming from MySQL.
pub fn from_mysql_to_dt_3_utc_opt(x: mysql::Value) -> Option<DateTime<Utc>> {
    match x {
        mysql::Value::NULL => return None,
        _ => {}
    }

    from_mysql_to_dt_3_utc(x).into()
}

pub fn from_mysql_to_dt_3_utc(x: mysql::Value) -> DateTime<Utc> {
    // Use UniCon.DT as the trait orphan rules prevent a direct mysql::Value->DateTime<Utc>
    let x2: DT = x.into();
    let x3: DateTime<Utc> = x2.into();

    x3.into()
}


// Copy technique at mysql_common-0.24.1/src/value/convert/engines:287
#[derive(Debug, Clone, PartialEq)]
pub struct ParseIr<T> {
    value: mysql::Value,
    output: T,
}

impl ConvIr<DT> for ParseIr<DT> {
    fn new(v: mysql::Value) -> Result<ParseIr<DT>, FromValueError> {
        match v {
            // mysql::Value::NULL - This should be handled by the parent `Option` type (before executing this branch with a mysql::Value).

            mysql::Value::Date(year, month, day, hour, min, sec, micro) => {
                Ok(ParseIr {
                    value: v.clone(),
                    output: v.into(),
                })
            }

            v => Err(FromValueError(v)),
        }
    }
    fn commit(self) -> DT {
        self.output
    }
    fn rollback(self) -> mysql::Value {
        self.value
    }
}


impl FromValue for DT {
    type Intermediate = ParseIr<DT>;

    fn from_value(v: mysql::Value) -> DT {
        <_>::from_value_opt(v).expect("Could not retrieve DT from Value")
    }
}

