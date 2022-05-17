use chrono::{DateTime, TimeZone, Utc};
//use unicon::dt::DT;
//use unicon::dt3::DT3;
use mysql::serde_json::Value;
use serde::Serialize;
use stripe_client::types::types::GetId;
use unicon::{*};
use unicon::dt::{*};
use unicon::dt3::{*};
use unicon::engines::mysql::{*};
use unicon::engines::postgres::{*};
use unicon::engines::placeholder::{*};
use unicon::engines::sqlite::{*};
use unicon::table::{*};
use unicon::traits::{*};
use unicon::uc::{*};
use unicon::utx::{*};
use unicon_proc_macro::{*};

/// Converts an array of objects into an array of primary keys.
/// - To be stored in a SQL column of type JSON.
///     - As a list of foreign keys (instead of having another relation table).
// trait ArrayPKs where Self: Iterator, Self::Item: GetId {
//     fn get_pks(&self) -> Option<String> {
//         // Issue: No trait for just `iter()`?
//         let ids: Vec<String> = self.iter().map(|x| x.get_id()).collect();
//         json_string_or_none_opt(&ids)
//     }
// }
pub trait ArrayPKs {
    fn get_pks_json_opt(&self) -> Option<String>;
    fn get_pks_json(&self) -> String;
}


impl<T> ArrayPKs for Vec<T> where T: GetId {
    /// Used when the array coming from the network type can be null.
    fn get_pks_json_opt(&self) -> Option<String> {
        let ids: Vec<String> = self.iter().map(|x| x.get_id()).collect();
        json_string_or_none_opt(&ids)
    }

    /// Used when the array coming from the network type is never null (`[]` written to SQL table for no items).
    fn get_pks_json(&self) -> String {
        let ids: Vec<String> = self.iter().map(|x| x.get_id()).collect();
        json_string(&ids)
    }
}

pub trait ToISODate {
    /// Returns a date in the format `2020-01-01 01::01:01`
    fn to_iso(&self) -> String;
    fn to_dt(&self) -> DateTime<Utc>;
}

impl ToISODate for i64 {
    fn to_iso(&self) -> String {
        unix_to_iso(self.clone())
    }
    fn to_dt(&self) -> DateTime<Utc> {
        unix_to_dt(self.clone())
    }
}


pub trait ToDT {
    fn to_dt(self) -> DT;
    fn to_dt3(self) -> DT3;
}

// Unix timestamp.
impl ToDT for i64 {
    fn to_dt(self) -> DT {
        Utc.timestamp(self, 0).naive_utc().into()
    }

    fn to_dt3(self) -> DT3 {
        Utc.timestamp(self, 0).naive_utc().into()
    }
}


pub trait ToJSONKey {
    fn to_json_key(&self) -> String;
}

pub trait ToJSONKeyOrNone {
    fn to_json_key_or_none(&self) -> Option<String>;
}

impl<T> ToJSONKey for T where T: Serialize {
    fn to_json_key(&self) -> String {
        json_key(self)
    }
}

impl<T> ToJSONKeyOrNone for Option<T> where T: Serialize + ToJSONKey {
    fn to_json_key_or_none(&self) -> Option<String> {
        self.as_ref()?.to_json_key().into()
    }
}


pub trait ToJSONString {
    fn to_json(&self) -> String;
}

impl<T> ToJSONString for T where T: Serialize {
    fn to_json(&self) -> String {
        json_string(self)
    }
}

pub trait ToJSONOrNone {
    fn to_json_or_none(&self) -> Option<String>;
}

impl<T> ToJSONOrNone for T where T: Serialize {
    /// Returns `None` for empty objects, arrays or null.
    fn to_json_or_none(&self) -> Option<String> {
        json_string_or_none_opt(self)
    }
}


// Helper function to allow dot notation/more consise code.
pub trait ToVal {
    fn json(&self) -> serde_json::Value;
}

impl<T> ToVal for T where T: Serialize {
    fn json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

pub trait ToValOrNone {
    fn json_or_none(&self) -> Option<serde_json::Value>;
}

impl<T> ToValOrNone for T where T: Serialize {
    /// Returns `None` for empty objects or non-js-object types (string, int, null).
    /// - This function was originally named `to_json_or_none`.
    ///     - On rename, empty arrays are allowed (instead of mapping to None).
    fn json_or_none(&self) -> Option<serde_json::Value> {
        let v = serde_json::to_value(self).unwrap();
        match &v {
            Value::Array(_) => return v.into(),
            Value::Object(x) if x.keys().len() > 0 => return v.into(),
            _ => {}
        }

        None
    }
}


// /// @todo/low Convert metedata {} into {}, not null, as this adds more possible state for clients to handle (vs checking a hashmap for a key)?
// impl<T> ToJSONOrNone for Option<T> where T: Serialize {
//     fn to_json_or_none(&self) -> Option<String> {
//         // If self.is_none(), short circuit immediately (instead of converting Option<T> -> "null" and string matching "null" into a None).
//         json_string_or_none_opt(self.as_ref()?)
//     }
// }


/// Replaces:
/// `payment_method_details_type: i.payment_method_details.as_ref().and_then(|x| x.type_x.clone().into())`
/// With:
/// `payment_method_details_type: i.payment_method_details.pick_opt(|x| &x.type_x)`
pub trait PickOpt<F, X, R> where {
    fn pick_opt(&self, f: F) -> Option<R>;
}

impl<F, X, R> PickOpt<F, X, R> for Option<X> where F: Fn(&X) -> &R, R: Clone {
    fn pick_opt(&self, f: F) -> Option<R> {
        f(self.as_ref()?).clone().into()
    }
}


// @todo/low Move to utils.
pub fn json_string<T: Serialize>(x: &T) -> String {
    serde_json::to_string(x).unwrap()
}

/// Note: Empty objects or arrays are replaced with SQL null (no need to parse JSON to see if it has values).
/// When None, the resulting SQL cell is SQL::Null.
/// - Is it better to use Null or `{}`?
pub fn json_string_or_none<T: Serialize>(x: &Option<T>) -> Option<String> {
    let json_str = serde_json::to_string(x.as_ref()?).unwrap();

    // Use SQL null for empty objects.
    // - Easier to find values that are actually set (both visually and with a JSON API).
    // - Stripe's Open API spec declares `metadata` as `nullable`, but returns an empty object.
    if vec!["[]", "{}", "null"].contains(&json_str.as_str()) {
        return None;
    }

    Some(json_str)
}

pub fn json_string_or_none_opt<T: Serialize>(x: &T) -> Option<String> {
    let json_str = serde_json::to_string(x).unwrap();

    // Use SQL null for empty objects.
    // - Easier to find values that are actually set (both visually and with a JSON API).
    // - Stripe's Open API spec declares `metadata` as `nullable`, but returns an empty object.
    if vec!["[]", "{}", "null"].contains(&json_str.as_str()) {
        return None;
    }

    Some(json_str)
}


/// When: Converting an enum into its JSON string key.
/// E.g. `"some_key"` => `some_key`
pub fn json_key<T: Serialize>(x: &T) -> String {
    serde_json::to_string(x).unwrap()
        .as_str()
        .strip_prefix("\"").unwrap()
        .strip_suffix("\"").unwrap()
        .to_string()
}

/// E.g. 1609433762 ->  `2020-01-01 01:01:01`
/// @todo/low Move this to the SQL derive macro, allow using DateTime object.
pub fn unix_to_iso(ts: i64) -> String {
    let dt = Utc.timestamp(ts, 0);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn unix_to_iso_wrap(ts: Option<i64>) -> Option<String> {
    let dt = Utc.timestamp(ts?, 0);
    dt.format("%Y-%m-%d %H:%M:%S").to_string().into()
}

pub fn unix_to_dt(ts: i64) -> DateTime<Utc> {
    let dt = Utc.timestamp(ts, 0);
    dt
}


/// Flatten/clone a field from a child struct into a parent.
/// - Parent field must be an Option.
/// - Child field must be an Option.
/// - If either are None, returns None.
/// @todo/low Place into trait, use generic impl to apply to all `Option<T>` to enable `s1.clone_field(\x\ x.field1)` usage.
/// @todo/med Move to "util" in uc_lib.
pub fn f<T, X: Clone>(p: &Option<T>, f: fn(&T) -> &Option<X>) -> Option<X> {
    f(p.as_ref()?).clone().into()
}

/// Wraps non-option closure return value in an option.
/// Note: `?` only works with explicit `Option` and will not work with generic?
/// - If `?` is used then an explicit `Option` type signature must be used - resulting in two of every function (one for `Option`, one without).
pub fn f_opt<T, X: Clone>(p: &Option<T>, f: fn(&T) -> &X) -> Option<X> {
    f(p.as_ref()?).clone().into()
}


/// Allows using the `?` to unwrap inline by wrapping the expression in a closure.
pub fn x<F, T>(f: F) -> Option<T> where F: Fn() -> Option<T> {
    f()
}
