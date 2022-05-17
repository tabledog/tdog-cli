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

// Wrap to enable implementing traits for (sql_engine_x_trait, NaiveDateTime).
// Type name is DT{X} as from/to traits never pass metadata (so meta data must be passed in the type name, like a 3-digit-ms resolution).
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct DT3 {
    pub dt: NaiveDateTime,
}

// Enable parent type to have default derived, even though this value may always be set on struct instantiation.
impl Default for DT3 {
    fn default() -> Self {
        DT3 { dt: NaiveDateTime::from_timestamp(0, 0) }
    }
}


impl From<NaiveDateTime> for DT3 {
    fn from(dt: NaiveDateTime) -> Self {
        DT3 { dt }
    }
}

impl From<DT3> for NaiveDateTime {
    fn from(x: DT3) -> Self {
        x.dt
    }
}


impl From<DateTime<Utc>> for DT3 {
    fn from(x: DateTime<Utc>) -> Self {
        DT3 {
            dt: x.naive_utc()
        }
    }
}

impl From<DT3> for DateTime<Utc> {
    fn from(x: DT3) -> Self {
        Utc.from_utc_datetime(&x.dt)
    }
}


// Note: `rusqlite` has a feature `chrono` that implements this.
// - But it does not enforce a certain resolution?
//      - Or: It is unlikely that by default 3+ SQL engines will implement the to/from formatting to ms resolution by default (and/or provide options to give a custom resolution).
// @see https://github.com/rusqlite/rusqlite/blob/master/src/types/chrono.rs
impl rusqlite::types::ToSql for DT3 {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        let iso_string = self.dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
        Ok(ToSqlOutput::Owned(Value::Text(iso_string)))
    }
}


impl rusqlite::types::FromSql for DT3 {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let dt3 = match value {
            ValueRef::Text(x) => {
                use std::str;
                let dt = NaiveDateTime::parse_from_str(str::from_utf8(x).unwrap(), "%Y-%m-%d %H:%M:%S.%3f").unwrap();
                DT3 { dt }
            }
            _ => unreachable!("Cannot convert non-SQLite-text col to DT3.")
        };

        Ok(dt3)
    }
}

use postgres::types::{to_sql_checked, Type, IsNull};
use postgres::types::private::BytesMut;
use std::error::Error;

impl postgres::types::ToSql for DT3 {

    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
        self.dt.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        NaiveDateTime::accepts(ty)
    }

    // Used internally by crate rust-postgres.
    to_sql_checked!();
}


impl postgres::types::FromSql<'_> for DT3 {
    fn from_sql(ty: &Type, raw: &'_ [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Note: this is microseconds (not milliseconds, DT*3*).
        let dt = NaiveDateTime::from_sql(ty, raw)?;
        Ok(DT3 { dt })
    }

    fn accepts(ty: &Type) -> bool {
        NaiveDateTime::accepts(ty)
    }
}



impl From<DT3> for mysql::Value {
    fn from(dt3: DT3) -> Self {
        // year, month, day, hour, minutes, seconds, micro seconds

        let (year, month, day, hour, min, sec, micro) = (
            dt3.dt.year() as u16,
            dt3.dt.month() as u8,
            dt3.dt.day() as u8,
            dt3.dt.hour() as u8,
            dt3.dt.minute() as u8,
            dt3.dt.second() as u8,
            dt3.dt.timestamp_subsec_micros() as u32,
        );

        let iso_string = dt3.dt.format("%Y-%m-%d %H:%M:%S.%3f").to_string();

        // dbg!(micro);
        // dbg!(iso_string);

        mysql::Value::Date(year, month, day, hour, min, sec, micro)
    }
}


impl From<mysql::Value> for DT3 {
    fn from(x: mysql::Value) -> Self {
        match x {
            mysql::Value::Date(year, month, day, hour, min, sec, micro) => {
                DT3 {
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
    // Use UniCon.DT3 as the trait orphan rules prevent a direct mysql::Value->DateTime<Utc>
    let x2: DT3 = x.into();
    let x3: DateTime<Utc> = x2.into();

    x3.into()
}


// Copy technique at mysql_common-0.24.1/src/value/convert/engines:287
#[derive(Debug, Clone, PartialEq)]
pub struct ParseIr<T> {
    value: mysql::Value,
    output: T,
}

impl ConvIr<DT3> for ParseIr<DT3> {
    fn new(v: mysql::Value) -> Result<ParseIr<DT3>, FromValueError> {
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
    fn commit(self) -> DT3 {
        self.output
    }
    fn rollback(self) -> mysql::Value {
        self.value
    }
}


impl FromValue for DT3 {
    type Intermediate = ParseIr<DT3>;

    fn from_value(v: mysql::Value) -> DT3 {
        <_>::from_value_opt(v).expect("Could not retrieve DT3 from Value")
    }
}

