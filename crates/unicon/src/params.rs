use rusqlite::ToSql;


// This file provides a single interface to all of the libraries param binding interfaces.
// - The aim is to allow the user to provide a `((&str, &T), ...` which represents ordered key/value's for creating a SQL statement and then binding values to the placeholders in the right order.
// - The order is important as indexing placeholders by their position is the lowest common denominator for SQL dialects (Postgres does not support key value placeholders).


// Allow referencing the sum of these traits cleanly.
// pub trait AnyParam: rusqlite::ToSql + std::clone::Clone + Into<mysql::Value> + postgres::types::ToSql + std::marker::Sync {}
// impl<T> AnyParam for T where T: rusqlite::ToSql + std::clone::Clone + Into<mysql::Value> + postgres::types::ToSql + std::marker::Sync {}


pub trait ToWhere: ToWhereIndexedSQLite + ToWhereIndexedPostgres + ToVecParamsSQLite + ToVecParamsMySQL + ToVecParamsPostgres {}
impl<T> ToWhere for T where T: ToWhereIndexedSQLite + ToWhereIndexedPostgres + ToVecParamsSQLite + ToVecParamsMySQL + ToVecParamsPostgres {}


pub trait ToWhereIndexedSQLite {
    fn to_where(&self) -> String;
}

// Use SQLite as index based parameter syntax is the same.
pub trait ToWhereIndexedMySQL {
    fn to_where(&self) -> String;
}

pub trait ToWhereIndexedPostgres {
    fn to_where(&self) -> String;
}


pub trait ToVecParamsSQLite {
    fn to_vec(&self) -> Vec<&dyn ToSql>;
}

pub trait ToVecParamsMySQL {
    fn to_vec(&self) -> Vec<mysql::Value>;
}

pub trait ToVecParamsPostgres {
    fn to_vec(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)>;
}

// One param
impl<T> ToVecParamsSQLite for (&str, &T)
    where T: ToSql {
    fn to_vec(&self) -> Vec<&dyn ToSql> {
        vec![
            self.1 as &dyn ToSql
        ]
    }
}

impl<T> ToVecParamsMySQL for (&str, &T)
    where T: Into<mysql::Value> + Clone {
    fn to_vec(&self) -> Vec<mysql::Value> {
        vec![
            self.1.into()
        ]
    }
}

impl<T> ToVecParamsPostgres for (&str, &T)
    where T: postgres::types::ToSql + Sync {
    fn to_vec(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)> {
        vec![
            self.1 as &(dyn postgres::types::ToSql + Sync)
        ]
    }
}

impl<T> ToWhereIndexedSQLite for (&str, &T) {
    fn to_where(&self) -> String {
        // @todo/low For MySQL, when the type is JSON, use `CAST(x as JSON)` in WHERE.
        format!("{} = ?", &self.0)
    }
}

impl<T> ToWhereIndexedPostgres for (&str, &T) {
    fn to_where(&self) -> String {
        format!("{} = $1", &self.0)
    }
}


// Two params.
impl<T1, T2> ToVecParamsSQLite for ((&str, &T1), (&str, &T2))
    where T1: ToSql,
          T2: ToSql
{
    fn to_vec(&self) -> Vec<&dyn ToSql> {
        vec![
            self.0.1 as &dyn ToSql,
            self.1.1 as &dyn ToSql,
        ]
    }
}

impl<T1, T2> ToVecParamsMySQL for ((&str, &T1), (&str, &T2))
    where T1: Into<mysql::Value> + Clone,
          T2: Into<mysql::Value> + Clone {
    fn to_vec(&self) -> Vec<mysql::Value> {
        vec![
            self.0.1.into(),
            self.1.1.into(),
        ]
    }
}


impl<T1, T2> ToVecParamsPostgres for ((&str, &T1), (&str, &T2))
    where T1: postgres::types::ToSql + Sync,
          T2: postgres::types::ToSql + Sync
{
    fn to_vec(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)> {
        vec![
            self.0.1 as &(dyn postgres::types::ToSql + Sync),
            self.1.1 as &(dyn postgres::types::ToSql + Sync),
        ]
    }
}


impl<T1, T2> ToWhereIndexedSQLite for ((&str, &T1), (&str, &T2)) {
    fn to_where(&self) -> String {
        format!("{} = ? AND {} = ?", &self.0.0, &self.1.0)
    }
}


impl<T1, T2> ToWhereIndexedPostgres for ((&str, &T1), (&str, &T2)) {
    fn to_where(&self) -> String {
        format!("{} = $1 AND {} = $2", &self.0.0, &self.1.0)
    }
}


// Three params.
impl<T1, T2, T3> ToVecParamsSQLite for ((&str, &T1), (&str, &T2), (&str, &T3))
    where T1: ToSql,
          T2: ToSql,
          T3: ToSql
{
    fn to_vec(&self) -> Vec<&dyn ToSql> {
        vec![
            self.0.1 as &dyn ToSql,
            self.1.1 as &dyn ToSql,
            self.2.1 as &dyn ToSql,
        ]
    }
}

impl<T1, T2, T3> ToVecParamsMySQL for ((&str, &T1), (&str, &T2), (&str, &T3))
    where T1: Into<mysql::Value> + Clone,
          T2: Into<mysql::Value> + Clone,
          T3: Into<mysql::Value> + Clone

{
    fn to_vec(&self) -> Vec<mysql::Value> {
        vec![
            self.0.1.into(),
            self.1.1.into(),
            self.2.1.into(),
        ]
    }
}


impl<T1, T2, T3> ToVecParamsPostgres for ((&str, &T1), (&str, &T2), (&str, &T3))
    where T1: postgres::types::ToSql + Sync,
          T2: postgres::types::ToSql + Sync,
          T3: postgres::types::ToSql + Sync
{
    fn to_vec(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)> {
        vec![
            self.0.1 as &(dyn postgres::types::ToSql + Sync),
            self.1.1 as &(dyn postgres::types::ToSql + Sync),
            self.2.1 as &(dyn postgres::types::ToSql + Sync),
        ]
    }
}


impl<T1, T2, T3> ToWhereIndexedSQLite for ((&str, &T1), (&str, &T2), (&str, &T3)) {
    fn to_where(&self) -> String {
        format!("{} = ? AND {} = ? AND {} = ?", &self.0.0, &self.1.0, &self.2.0)
    }
}


impl<T1, T2, T3> ToWhereIndexedPostgres for ((&str, &T1), (&str, &T2), (&str, &T3)) {
    fn to_where(&self) -> String {
        format!("{} = $1 AND {} = $2 AND {} = $3", &self.0.0, &self.1.0, &self.2.0)
    }
}
