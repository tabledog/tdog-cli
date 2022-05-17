#![allow(warnings)]

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
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
use crate::table::{Table, ObjType, CreateSQLObj, StaticSQLStrings};
use crate::uc::{TableCreate, Schema, NameCreate};
use std::hint::unreachable_unchecked;
use log::debug;

pub enum UniTx<'a> {
    Rusqlite(Transaction<'a>),
    MySQL(mysql::Transaction<'a>),
    Postgres(postgres::Transaction<'a>),
    PlaceholderLibA(String),
}

impl UniTx<'_> {
    /// Note: Rusqlite defaults to ROLLBACK on drop, ensure other libs have the same meaning.
    pub fn tx_close(mut self) -> Result<()> {
        match self {
            UniTx::Rusqlite(tx) => {
                tx.commit();
                Ok(())
            }
            UniTx::MySQL(tx) => {
                tx.commit();
                Ok(())
            }
            UniTx::Postgres(tx) => {
                tx.commit();
                Ok(())
            }
            UniTx::PlaceholderLibA(_) => {
                0;
                Ok(())
            }
        }
    }

    pub fn tx_rollback(mut self) -> Result<()> {
        match self {
            UniTx::Rusqlite(tx) => {
                tx.rollback();
                Ok(())
            }
            UniTx::MySQL(tx) => {
                tx.rollback();
                Ok(())
            }
            UniTx::Postgres(tx) => {
                tx.rollback();
                Ok(())
            }
            UniTx::PlaceholderLibA(_) => {
                0;
                Ok(())
            }
        }
    }


    // Executes one SQL statement that does not need params (E.g. create table|index).
    // - fn return indicates no error;
    pub fn exec_one(&mut self, sql: &str) {
        match self {
            UniTx::Rusqlite(tx) => {
                tx.execute(sql, NO_PARAMS).unwrap();
            }
            UniTx::MySQL(tx) => {
                tx.query_drop(sql).unwrap();
            }
            UniTx::Postgres(tx) => {
                tx.execute(sql, &[]).unwrap();
            }
            UniTx::PlaceholderLibA(_) => unreachable!()
        };

        // Does not return changes (requires extra query for MySQL).
    }

    // pub fn exec_one_changes

    // @todo/maybe: Use params that mirror UniCon, UniTx?
    // pub fn exec_one_params<T: IntoStdParamsEnum>(&mut self, sql: &str, params: T) {
    // }


    pub fn get_last_id(&self) -> i64 {
        match self {
            UniTx::Rusqlite(tx) => {
                let x = tx.last_insert_rowid();
                assert!(x > 0, "Expected to the last insert id from SQLite, found 0.");
                x
            }
            UniTx::MySQL(tx) => {
                i64::try_from(tx.last_insert_id().unwrap()).unwrap()
            }
            UniTx::Postgres(tx) => {
                unreachable!("Cannot `utx.get_last_id()` for Postgres as there is no way to reliably read it in the presence of triggers. See: https://stackoverflow.com/a/2944481/4949386 Fix: Use the ID returned from `insert` which uses `INSERT ... RETURNING id`");
            }
            UniTx::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }


    pub fn get_active_schema(&mut self) -> Option<String> {
        match self {
            UniTx::Rusqlite(tx) => {
                None
            }
            UniTx::MySQL(tx) => {
                let x: Option<Option<String>> = tx.query_first("SELECT DATABASE()").unwrap();
                x.unwrap()
            }
            UniTx::Postgres(tx) => {
                // @see https://www.postgresql.org/docs/9.6/ddl-schemas.html
                // - First schema in search path is where new objects are created.
                let res = tx.query("SHOW search_path", &[]).unwrap();
                let row = res.first().unwrap();
                // Default = `"$user", public`
                let search_path: String = row.get(0);
                let parts: Vec<_> = search_path.split(",").map(|x| x.trim()).collect();
                if parts.len() > 0 {
                    parts[0].to_string().into()
                } else {
                    None
                }
            }
            UniTx::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }
    pub fn set_active_schema(&mut self, schema: &String) {
        match self {
            UniTx::Rusqlite(tx) => {
                unreachable!("Cannot set schema for SQLite");
            }
            UniTx::MySQL(tx) => {
                // Note: This is not isolated; it sets the connection schema (not just the tx).
                tx.query_drop(format!("USE {}", schema)).unwrap();
            }
            UniTx::Postgres(tx) => {
                // Note: No `"$user"`
                // If tx is committed, this persists; else it only applies until the end of the tx.
                // - @see https://www.postgresql.org/docs/9.1/sql-set.html#:~:text=If%20SET%20(or%20equivalently%20SET%20SESSION)%20is%20issued%20within%20a%20transaction%20that%20is%20later%20aborted%2C%20the%20effects%20of%20the%20SET%20command%20disappear%20when%20the%20transaction%20is%20rolled%20back
                tx.query(format!("SET search_path TO {}, public", schema).as_str(), &[]).unwrap();
            }
            UniTx::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }

    pub fn get_schemas(&mut self) -> Vec<String> {
        match self {
            UniTx::Rusqlite(tx) => {
                // No schemas for SQLite (this is the file).
                vec![]
            }
            UniTx::MySQL(tx) => {
                tx.query("SHOW SCHEMAS").unwrap()
            }
            UniTx::Postgres(tx) => {
                let mut v = vec![];
                for x in tx.query("select schema_name from information_schema.schemata", &[]).unwrap() {
                    v.push(x.get(0));
                }
                // dbg!(&v);
                v
            }
            UniTx::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }


    pub fn get_tables(&mut self) -> Vec<String> {
        // This should exist and be set and before calling `get_tables` (a schema is needed to read tables from).
        let target_schema = self.get_active_schema();

        match self {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached("SELECT name FROM sqlite_master WHERE type='table'").unwrap();
                let mut rows = stmt.query_map(NO_PARAMS, |r| {
                    r.get(0)
                }).unwrap();

                let mut set = vec![];
                for r in rows {
                    set.push(r.unwrap());
                };
                return set.into_iter().filter(|x| x != "sqlite_sequence").collect();
            }
            UniTx::MySQL(tx) => {
                // `USE target_schema` already set on connection
                tx.query("SHOW TABLES").unwrap()
            }
            UniTx::Postgres(tx) => {
                let schema = target_schema.expect("Postgres should have an active schema readable from search_path prefix.");
                debug!("Postgres: reading tables from schema `{}`", schema);
                let q = "SELECT tablename, schemaname FROM pg_catalog.pg_tables WHERE schemaname = $1";
                let rows = tx.query(q, &[&schema]).unwrap();
                rows.iter().map(|x| x.get(0)).collect::<Vec<String>>()
            }
            UniTx::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }


    // Reads (schema, tables and indexes) from a given schema.
    // - Used when comparing a target schema with a possibly existing one.
    pub fn get_existing_schema(&mut self, target_schema_opt: Option<&str>) -> Schema {
        let get_tables = |utx: &mut UniTx| -> Vec<TableCreate> {
            utx.get_tables().iter().map(|x| TableCreate {
                name: x.clone(),

                // @todo/low if needed in the future (only doing basic name-name compare atm, not structure comparison).
                create: "".to_string(),
                indexes: vec![],
            }).collect()
        };

        match self {
            UniTx::Rusqlite(tx) => {
                assert_eq!(target_schema_opt, None);

                return Schema {
                    schema: None,
                    tables: get_tables(self),
                };
            }
            UniTx::MySQL(_) | UniTx::Postgres(_) => {
                let target_schema = target_schema_opt.unwrap().to_string();

                let orig_active = self.get_active_schema();
                let schemas = self.get_schemas();

                let schema_exists = schemas.contains(&target_schema);

                // At this point the schema may not have been created.
                if !schema_exists {
                    return Schema {
                        schema: None,
                        tables: vec![],
                    };
                }

                if orig_active != Some(target_schema.clone()) {
                    self.set_active_schema(&target_schema);
                }

                let s = Schema {
                    schema: Some(NameCreate {
                        name: target_schema.clone(),
                        create: "".to_string(),
                    }),
                    tables: get_tables(self),
                };


                // Revert to previous `USE x`.
                // - There is no `UNUSE` (cannot go back to NULL).
                // - In case of a relevant connection level USE that subsequent code relies on.
                //      - E.g. Take a USE x from a config, apply it to a connection on create, loop over many different un-related schemas to the same MySQL connection.
                // - @todo/low A default schema that is set before each con/tx operation?
                if let Some(x) = orig_active {
                    self.set_active_schema(&x);
                }

                s
            }
            UniTx::PlaceholderLibA(_) => unreachable!()
        }
    }
}