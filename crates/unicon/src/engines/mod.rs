// This mod is for combinations of [engine, lib], for example [SQLite, Rusqlite].
// - Note: in the future there may be many clients for the same db engine (e.g. an async and sync API based on different dependencies).


// Google Trends relative search frequency, 2021-Jun:
// - 33% SQL server
// - 32% MySQL
// - 16% Oracle
// - 14% Postgres
// - 5% SQLite


pub mod sqlite;
pub mod mysql;
pub mod postgres;
pub mod placeholder;
