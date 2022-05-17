#![allow(warnings)]

use serde::{Serialize, Deserialize};

extern crate proc_macro;
// extern crate proc_macro2;


#[macro_use]
extern crate lazy_static;
extern crate regex;

use proc_macro::TokenStream;
use std::{thread, time};
use std::any::Any;
use std::convert::TryFrom;
use std::iter::{Enumerate, Map};
use std::slice::Iter;

use quote::__private::TokenTree;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use syn;
use syn::{Data, DataEnum, DataStruct, Field, Fields, GenericArgument, Ident, LitStr, Meta, parse_macro_input, PathArguments, Type, Attribute};
use syn::export::TokenStream2;

use unicon::table::*;


#[proc_macro_derive(Insert, attributes(table_name_plural, primary_key, unique, index, col_type, skip, update_ts, insert_ts, fk))]
pub fn insert(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_insert(&ast)
}


// #[proc_macro_derive(Table)]
// pub fn table(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();
//     impl_table(&ast)
// }


// @todo/low How can the same attributes be used for both `Insert` and `ColMeta` without duplication or compile error?
// #[proc_macro_derive(ColMeta, attributes(primary_key, unique, index, col_type))]
// pub fn col_meta_derive(input: TokenStream) -> TokenStream {
//     // Note: Does not modify AST.
//     TokenStream::new()
// }


#[proc_macro_derive(SQLiteString)]
pub fn sqlite_string_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_sqlite_string(&ast)
}

// #[proc_macro_derive(SQLiteCreateAll)]
// pub fn sqlite_string_schema_derive(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     impl_sqlite_string_schema(&ast)
// }


#[proc_macro_derive(MySQLString)]
pub fn mysql_string_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mysql_string(&ast)
}

// #[proc_macro_derive(MySQLCreateAll)]
// pub fn mysql_string_schema_derive(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     impl_mysql_string_schema(&ast)
// }


#[proc_macro_derive(Db)]
pub fn db_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_db(&ast)
}


#[proc_macro_derive(SQLiteFuncRusqlite)]
pub fn sqlite_func_rusqlite_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_sqlite_func_rusqlite(&ast)
}


// #[proc_macro_derive(PlaceholderString)]
// pub fn placeholder_string_derive(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     impl_placeholder_string(&ast)
// }


#[proc_macro_derive(PlaceholderFuncStd)]
pub fn placeholder_func_std_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_placeholder_func_std(&ast)
}


// Tip: Use option+enter, fill match arms to get possible options at each node in the tree.
fn impl_sqlite_string(ast: &syn::DeriveInput) -> TokenStream {
    let table: Table = ast.into();

    let sname = format_ident!("{}", table.struct_name);

    // dbg!(&table);
    // let c: SQLiteCreate = table.into();

    // let c = <Table as ToSQLString<SQLite>>::get_create_table(&table);
    // let i = <Table as ToSQLString<SQLite>>::get_insert(&table);
    // let indexes = <Table as ToSQLString<SQLite>>::get_create_indexes(&table);


    let assert_fields_exist = table.indexes.iter().map(|i| {
        // Assert: Index has a unique name per schema.
        let f_name = format_ident!("_sql_index_sqlite_{}", i.name);
        let s_name = format_ident!("{}", table.struct_name);
        let fields = i.fields_used.iter().map(|f| {
            let i = format_ident!("{}", f);
            quote! {
                x.#i
            }
        });

        quote! {
            // Note: Auto generated function that asserts `create index` statements reference valid struct fields at compile time.
            fn #f_name(x: #s_name) {
                || {
                    ( #(#fields),* )
                };
            }
        }
    });

    let gen = quote! {
        // impl SQLiteString for #sname {
        //     fn get_insert(&self) -> &'static str {
        //         return #i;
        //     }
        //
        //     fn get_create_table(&self) -> &'static str {
        //         return #c;
        //     }
        //     fn get_create_indexes(&self) -> Vec<&'static str> {
        //         vec![
        //             #(#indexes),*
        //         ]
        //     }
        // }

        // impl SQLiteString for &#sname {
        //     fn get_insert(&self) -> &'static str {
        //         return #i;
        //     }
        //
        //     fn get_create_table(&self) -> &'static str {
        //         return #c;
        //     }
        //     fn get_create_indexes(&self) -> Vec<&'static str> {
        //         vec![
        //             #(#indexes),*
        //         ]
        //     }
        // }

        #( #assert_fields_exist )*
    };

    gen.into()
}


fn impl_mysql_string(ast: &syn::DeriveInput) -> TokenStream {
    let table: Table = ast.into();

    let sname = format_ident!("{}", table.struct_name);

    let assert_fields_exist = table.indexes.iter().map(|i| {
        // Assert: Index has a unique name per schema.
        let f_name = format_ident!("_sql_index_mysql_{}", i.name);
        let s_name = format_ident!("{}", table.struct_name);
        let fields = i.fields_used.iter().map(|f| {
            let i = format_ident!("{}", f);
            quote! {
                x.#i
            }
        });

        quote! {
            // Note: Auto generated function that asserts `create index` statements reference valid struct fields at compile time.
            fn #f_name(x: #s_name) {
                || {
                    ( #(#fields),* )
                };
            }
        }
    });

    let gen = quote! {
        // impl MySQLString for #sname {
        //     fn get_insert(&self) -> &'static str {
        //         return #i;
        //     }
        //
        //     fn get_create_table(&self) -> &'static str {
        //         return #c;
        //     }
        //     fn get_create_indexes(&self) -> Vec<&'static str> {
        //         vec![
        //             #(#indexes),*
        //         ]
        //     }
        // }

        #( #assert_fields_exist )*
    };

    gen.into()
}


// fn impl_sqlite_string_schema(ast: &syn::DeriveInput) -> TokenStream {
//     let enum_name = format_ident!("{}", &ast.ident.to_string());
//
//     let r: Rows = (match &ast.data {
//         Data::Enum(x) => x.into(),
//         _ => panic!("Expected enum.")
//     });
//
//     let sql_create_strings = r.struct_names.iter().map(|n| {
//         let s_name = format_ident!("{}", n);
//
//         quote! {
//             // x.append(&mut <#s_name as SQLiteString>::get_create(&#s_name::default()));
//
//             // Create table
//             x.push(<#s_name as TableStatic>::get_table().static_sql_strings.as_ref().unwrap().sqlite.create.clone());
//
//             // Create indexes
//             x.append(&mut <#s_name as TableStatic>::get_table().static_sql_strings.as_ref().unwrap().sqlite.indexes.clone());
//         }
//     });
//
//
//     let gen = quote! {
//         impl SQLiteCreateAll for #enum_name {
//             fn get_create_all() -> Vec<CreateSQLObj> {
//                 let mut x = vec![];
//
//                 #(#sql_create_strings);*
//
//                 x
//             }
//         }
//     };
//
//     gen.into()
// }


// fn impl_mysql_string_schema(ast: &syn::DeriveInput) -> TokenStream {
//     let enum_name = format_ident!("{}", &ast.ident.to_string());
//
//     let r: Rows = (match &ast.data {
//         Data::Enum(x) => x.into(),
//         _ => panic!("Expected enum.")
//     });
//
//     let sql_create_strings = r.struct_names.iter().map(|n| {
//         let s_name = format_ident!("{}", n);
//
//         quote! {
//              // <#s_name as MySQLString>::get_create(&#s_name::default())
//             // x.append(&mut <#s_name as MySQLString>::get_create(&#s_name::default()));
//
//                         // Create table
//             x.push(<#s_name as TableStatic>::get_table().static_sql_strings.as_ref().unwrap().mysql.create.clone());
//
//             // Create indexes
//             x.append(&mut <#s_name as TableStatic>::get_table().static_sql_strings.as_ref().unwrap().mysql.indexes.clone());
//         }
//     });
//
//
//     let gen = quote! {
//         impl MySQLCreateAll for #enum_name {
//             fn get_create_all() -> Vec<CreateSQLObj> {
//                 let mut x = vec![];
//
//                 #(#sql_create_strings);*
//
//                 x
//             }
//         }
//     };
//
//     gen.into()
// }


fn impl_db(ast: &syn::DeriveInput) -> TokenStream {
    let enum_name = format_ident!("{}", &ast.ident.to_string());

    let r: Rows = (match &ast.data {
        Data::Enum(x) => x.into(),
        _ => panic!("Expected enum.")
    });

    let sql_table_names = r.struct_names.iter().map(|n| {
        let s_name = format_ident!("{}", n);

        quote! {
             <#s_name as TableStatic>::get_table_name_static()
        }
    });


    let table_refs = r.struct_names.iter().map(|n| {
        let s_name = format_ident!("{}", n);

        quote! {
             <#s_name as TableStatic>::get_table()
        }
    });

    let gen = quote! {
        impl DbStatic for #enum_name {
            fn get_tables() -> Vec<&'static Table> {
                vec![
                    #(#table_refs),*
                ]
            }

            fn get_table_names() -> Vec<&'static str> {
                vec![
                    #(#sql_table_names),*
                ]
            }
        }
    };


    let mut s1: TokenStream = gen.into();

    let deps: Vec<TokenStream> = vec![
        // impl_sqlite_string_schema(ast),
        // impl_mysql_string_schema(ast)
    ];

    for d in deps {
        s1.extend(d)
    }

    s1
}


fn cols_to_vec_sqlite(cols: Vec<Col>) -> Vec<TokenStream2> {
    cols.iter().map(|c| {
        let k = format!("{}", c.name.clone());
        let p = format!(":{}", c.name.clone());
        let v = format_ident!("{}", c.name_raw);
        let assert_is_json_object = assert_is_json_object(&quote! {&self.#v}, &c);

        quote! {
            v.push(SQLiteKV {key: #k, key_param: #p, val: &self.#v as &dyn rusqlite::ToSql});
            #assert_is_json_object
        }
    }).collect()
}


fn cols_to_vec_postgres(cols: Vec<Col>) -> Vec<TokenStream2> {
    cols.iter().map(|c| {
        let k = format!("{}", c.name.clone());
        // let p = format!(":{}", c.name.clone());
        let v = format_ident!("{}", c.name_raw);
        let assert_is_json_object = assert_is_json_object(&quote! {&self.#v}, &c);

        quote! {
            // m.insert(#k, (#p, &self.#v as &(dyn postgres::types::ToSql + Sync)));

            v.push((#k, &self.#v as &(dyn postgres::types::ToSql + Sync)));

            #assert_is_json_object
        }
    }).collect()
}


fn cols_to_vec_mysql(cols: Vec<Col>) -> Vec<TokenStream2> {
    cols.iter().map(|c| {
        let k = format!("{}", &c.name);
        let v = format_ident!("{}", &c.name_raw);

        let assert_is_json_object = assert_is_json_object(&quote! {&self.#v}, &c);

        quote! {
            v.push(MySQLKV {key: #k.to_string(), val: self.#v.clone().into()});
            #assert_is_json_object
        }
    }).collect()
}


fn impl_sqlite_func_rusqlite(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());

    // @todo/low Cache and re-use this conversion (it is used to generate both the SQLiteString and SQLiteFunc* traits).
    let t: Table = ast.into();

    /// @todo/maybe Issue: Keeping the struct data and the row meta data sets in two different structs would make this more obvious. Instead of a flat Rust struct with (pk, insert_ts, update_ts).
    /// - A: Traits for getting/setting these values.
    /// - B: Use a `row_meta: MetaData{pk, insert_ts, update_ts}` field.
    let cols_all = cols_to_vec_sqlite(t.cols_not_skipped());
    let cols_writable = cols_to_vec_sqlite(t.cols_writable_only());

    let field_matches = t.cols.iter().map(|x| {
        let name = &x.name;
        let field = format_ident!("{}", x.name_raw);
        let assert_is_json_object = assert_is_json_object(&quote! {&a.#field}, &x);

        quote! {
            #name => {
                a.#field = r.get(r.column_index(col_name).unwrap()).unwrap();
                #assert_is_json_object
            },
        }
    });

    let gen = quote! {
        impl SQLiteFuncRusqlite for #s_name {

            fn to_kv_all(&self) -> Vec<SQLiteKV> {
                let mut v = vec![];
                #( #cols_all )*
                v
            }

            fn to_kv_writable_only(&self) -> Vec<SQLiteKV> {
                let mut v = vec![];
                #( #cols_writable )*
                v
            }

        }

        impl SQLiteFuncRusqliteStatic for #s_name {
            fn row_to_ins(r: &rusqlite::Row) -> Self {
                let mut a = #s_name {
                    ..Default::default()
                };
                for col_name in r.column_names() {
                    match col_name {
                        #( #field_matches )*
                        x => panic!(format!("SQL table field does not exist in Rust struct {}", x))
                    }
                }
                a
            }
        }

        // impl SQLiteFuncRusqlite for &#s_name {
        //     fn get_params_kv(&self) -> Vec<(&str, &dyn rusqlite::ToSql)> {
        //         vec![
        //                #(#kv2),*
        //         ]
        //     }
        // }
    };


    gen.into()
}

/// For JSON SQL cells, only allow JSON objects (objects and arrays).
/// - This prevents confusion around storing the JSON value "null" (instead of the SQL value NULL).
///     - SQL queries the end user writes should be clear.
fn assert_is_json_object(rust_field_access: &TokenStream2, col: &Col) -> TokenStream2 {
    let mut x = quote! {};
    let field_access_code = rust_field_access.to_string();

    if let RustType::Value = &col.t {
        let assert_is_obj = quote! {
            assert!(json_val.is_object() || json_val.is_array(), "Error: Only JSON objects or arrays can be stored in SQL cells - use native SQL types for scalar types. If `null` is needed, wrap the Rust struct field in `Option<serde_json::Value>`. Found JSON scalar type: `{:?}` when reading Rust struct field `{}`", json_val, #field_access_code);
        };

        x = if col.nullable {
            quote! {
                if let Some(json_val) = #rust_field_access {
                    #assert_is_obj
                }
            }
        } else {
            quote! {
                let json_val = #rust_field_access;
                #assert_is_obj
            }
        };
    }

    x
}


fn impl_mysql_func_x(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());

    // @todo/low Cache and re-use this conversion (it is used to generate both the SQLiteString and SQLiteFunc* traits).
    let t: Table = ast.into();

    let cols_all = cols_to_vec_mysql(t.cols_not_skipped());
    let cols_writable = cols_to_vec_mysql(t.cols_writable_only());


    let field_matches = t.cols.iter().map(|x| {
        let name = &x.name;
        let field = format_ident!("{}", x.name_raw);
        let assert_is_json_object = assert_is_json_object(&quote! {&a.#field}, &x);

        quote! {
            #name => {
                // a.#field = r.get(col_name).unwrap();
                a.#field = r.take(col_name).unwrap();
                #assert_is_json_object
            },
        }
    });


    let gen = quote! {
        impl MySQLFuncX for #s_name {

            fn to_kv_all(&self) -> Vec<MySQLKV> {
                let mut v = vec![];
                #( #cols_all )*
                v
            }

            fn to_kv_writable_only(&self) -> Vec<MySQLKV> {
                let mut v = vec![];
                #( #cols_writable )*
                v
            }

        }

        // Implement this to mirror other (Engine, LibX, Static) combos.
        impl MySQLFuncXStatic for #s_name {
            fn row_to_ins(mut r: &mut mysql::Row) -> Self {
                // Self::from_row_opt(r.clone()).unwrap()

                use std::ops::Deref;

                let mut a = #s_name {
                    ..Default::default()
                };
                for col in r.columns().iter() {
                    let cow = col.name_str();
                    let col_name: &str = cow.deref();
                    match col_name {
                        #( #field_matches )*
                        x => panic!(format!("SQL table field does not exist in Rust struct {}", x))
                    }
                }

                a
            }
        }

        // Issue: `FromValue` has to be implemented for `struct RowA` if implementing `FromRow`.
        // Issue; `FromValue` limited to arity 12 rows?
        // Implement this to allow seamless use with the `mysql` crate API.
        // impl mysql_common::row::convert::FromRow for #s_name {
        //     fn from_row_opt(r: mysql::Row) -> Result<Self, mysql_common::row::convert::FromRowError> where Self: Sized {
        //         use std::ops::Deref;
        //
        //         let mut a = #s_name {
        //             ..Default::default()
        //         };
        //         for col in r.columns().iter() {
        //             let cow = col.name_str();
        //             let col_name: &str = cow.deref();
        //             match col_name {
        //                 #( #field_matches )*
        //                 x => panic!(format!("SQL table field does not exist in Rust struct {}", x))
        //             }
        //         }
        //         Ok(a)
        //     }
        // }


    };


    gen.into()
}

fn impl_postgres_func_x(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());

    // @todo/low Cache and re-use this conversion (it is used to generate both the SQLiteString and SQLiteFunc* traits).
    let t: Table = ast.into();

    let cols_all = cols_to_vec_postgres(t.cols_not_skipped());
    let cols_writable = cols_to_vec_postgres(t.cols_writable_only());

    let field_matches = t.cols.iter().map(|x| {
        let name = &x.name;
        let field = format_ident!("{}", x.name_raw);
        let assert_is_json_object = assert_is_json_object(&quote! {&a.#field}, &x);

        quote! {
            #name => {
                // a.#field = r.get(col_name).unwrap();
                a.#field = r.get(col_name);
                #assert_is_json_object
            },
        }
    });


    let gen = quote! {
        impl PostgresFuncX for #s_name {

            fn to_kv_all(&self) -> Vec<(&'static str, &(dyn postgres::types::ToSql + Sync))> {
                let mut v = vec![];
                #( #cols_all )*
                v
            }

            fn to_kv_writable_only(&self) -> Vec<(&'static str, &(dyn postgres::types::ToSql + Sync))> {
                let mut v = vec![];
                #( #cols_writable )*
                v
            }

        }

        // Implement this to mirror other (Engine, LibX, Static) combos.
        impl PostgresFuncXStatic for #s_name {
            fn row_to_ins(r: &postgres::Row) -> Self {
                // Self::from_row_opt(r.clone()).unwrap()

                let mut a = #s_name {
                    ..Default::default()
                };
                for col in r.columns().iter() {
                    let col_name: &str = col.name();

                    match col_name {
                        #( #field_matches )*
                        x => panic!(format!("SQL table field does not exist in Rust struct {}", x))
                    }
                }

                a
            }
        }

    };


    gen.into()
}


fn impl_placeholder_string(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());
    let gen = quote! {
        impl PlaceholderString for #s_name {}
        // impl PlaceholderString for &#s_name {}
    };
    gen.into()
}

fn impl_placeholder_func_std(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());
    let gen = quote! {
        impl PlaceholderFuncStd for #s_name {}
        // impl PlaceholderFuncStd for &#s_name {}

        impl PlaceholderFuncStdStatic for #s_name {}
    };
    gen.into()
}

// @see https://stackoverflow.com/questions/67976782/how-do-i-transfer-an-instance-of-a-type-from-a-macro-to-the-compiled-program
fn impl_table(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());
    let t: Table = ast.into();

    let t_name = t.name.clone();
    // let fks = t.get_ts_fk_data();
    let assert_fields_exist = t.get_ts_fk_assert_fields_exist();


    let update_ts_k = if let Some(c) = t.cols_not_skipped().iter().find(|c| c.update_ts) {
        let n = c.name.clone();
        quote! {Some(#n)}
    } else {
        quote! {None}
    };

    let primary_key_key = t.get_primary_key_col_name();

    let t_str = serde_json::to_string(&t).unwrap();
    let t_json = quote! {
        serde_json::from_str(#t_str).unwrap()
    };

    let sr_name = format_ident!("TABLE_{}", &ast.ident.to_string().to_uppercase());
    let lazy_parse_json = quote! {
        lazy_static! {
            static ref #sr_name: Table = #t_json;
        }
    };


    let gen = quote! {
        #lazy_parse_json

        impl TableTr for #s_name  {
            fn get_table(&self) -> &Table {
                &#sr_name
            }

            fn get_table_name(&self) -> &'static str {
                #t_name
            }

            // fn get_fk_data(&self) -> Vec<FK> {
            //     vec![
            //         #(#fks),*
            //     ]
            // }

            fn get_key_update_ts(&self) -> Option<&'static str> {
                #update_ts_k
            }

            fn get_key_pk(&self) -> &'static str {
                #primary_key_key
            }
        }

        impl TableStatic for #s_name  {
            fn get_table() -> &'static Table {
                &#sr_name
            }

            fn get_table_name_static() -> &'static str {
                #t_name
            }
        }


        #( #assert_fields_exist )*
    };

    gen.into()
}


fn impl_insert(ast: &syn::DeriveInput) -> TokenStream {
    let s_name = format_ident!("{}", &ast.ident.to_string());

    let t: Table = ast.into();
    let pk = format_ident!("{}", t.get_primary_key_col_name());


    let gen = quote! {
        impl Insert for #s_name  {
            fn get_pk(&self) -> i64 {
                self.#pk.unwrap()
            }
            fn set_pk(&mut self, pk: i64) {
                self.#pk = Some(pk)
            }
        }

        impl QueryByStatic for #s_name {}

        // impl Insert for &#s_name  {
        //     fn get_pk(&self) -> i64 {
        //         self.#pk.unwrap()
        //     }
        //     fn set_pk(&mut self, pk: i64) {
        //         self.#pk = Some(pk)
        //     }
        // }
    };


    let mut s1: TokenStream = gen.into();

    // @todo/low How to use the `#[proc_macro_derive(X), ...]` meta data when composing macros?
    // - `proc_macro_derive` defines a public interface, and it seems the only way to enforce it at compile time is to use it on a struct (you cannot call it from within another macro and have the interface checked).

    // `#[derive(Table, ColMeta, SQLiteString, SQLiteFuncRusqlite)]`
    // `#[derive(PlaceholderString, PlaceholderFuncStd)]`
    let deps = vec![
        impl_table(ast),
        impl_sqlite_string(ast),
        impl_sqlite_func_rusqlite(ast),
        impl_mysql_string(ast),
        impl_mysql_func_x(ast),
        // impl_postgres_string(ast),
        impl_postgres_func_x(ast),
        impl_placeholder_string(ast),
        impl_placeholder_func_std(ast),
    ];


    for d in deps {
        s1.extend(d)
    }


    s1
}

