#![allow(warnings)]

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasherDefault;

use mysql::*;
use mysql::chrono::NaiveDateTime;
use mysql::prelude::*;
use regex::Regex;
use rusqlite::{Connection, NO_PARAMS, params, Result, Row, ToSql, Transaction, TransactionBehavior};
use rusqlite::ffi::Error;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use serde::{Deserialize, Deserializer, Serialize};
use twox_hash::XxHash;
use crate::engines::placeholder::{PlaceholderString, PlaceholderFuncStd, PlaceholderFuncStdStatic};
use crate::engines::mysql::{MySQLFuncX, MySQLFuncXStatic};
use crate::engines::sqlite::{SQLiteFuncRusqlite, SQLiteFuncRusqliteStatic};

pub mod dt3;
pub mod dt;
pub mod table;
pub mod engines;
pub mod uc;
pub mod utx;
pub mod traits;
mod data;
pub mod params;

// pub mod jo;
