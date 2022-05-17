
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

type PathString = Vec<(Vec<String>, String)>;

fn as_paths(v: &Value, inc_null: bool) -> PathString {
    fn visit(path: Vec<String>, v: &Value, ps: &mut PathString, inc_null: bool) {
        match v {
            Value::Object(x) => {
                for (k, v) in x {
                    let mut path = path.clone();
                    path.push(k.clone());
                    visit(path, v, ps, inc_null);
                }
            }
            Value::Array(x) => {
                for (i, v) in x.iter().enumerate() {
                    let mut path = path.clone();
                    path.push(i.to_string());
                    visit(path, v, ps, inc_null);
                }
            }
            Value::String(x) => {
                ps.push((path.clone(), format!("{}", x)));
            }
            Value::Number(x) => {
                ps.push((path.clone(), format!("{}", x)));
            }
            Value::Bool(x) => {
                ps.push((path.clone(), format!("{}", x)));
            }
            Value::Null => {
                if inc_null {
                    ps.push((path.clone(), "null".into()));
                }
            }
        }
    }

    let mut ps: PathString = vec![];
    visit(vec![], v, &mut ps, inc_null);
    ps
}

pub trait AsPaths: Serialize {
    fn as_paths(&self) -> PathString {
        let v: Value = serde_json::to_value(self).unwrap();
        as_paths(&v, false)
    }
}

impl<T> AsPaths for T where T: Serialize {}

fn flatten_paths(ps: PathString) -> Vec<(String, String)> {
    ps.iter()
        .map(|(path, v)| {
            // @todo/low Do array indexes need to be included, or just `[]`?

            // E.g. `k1[k2][k3][0][k4]`
            let k: String = path
                .iter()
                .enumerate()
                .map(|(i, v)| match i {
                    0 => v.clone(),
                    _ => format!("[{}]", v),
                })
                .collect::<Vec<String>>()
                .join("");

            (k, v.clone())
        })
        .collect()
}

/// @see https://swagger.io/docs/specification/serialization/
///
/// Store keys incase a different format is required, in which case a trait fn can be used.
/// OpenAPI parameter style=X types for url query strings.
/// Stripe v3 uses form, explode=true and deepObject which have the same format.
/// But form explode=false is different and could be used in other specs.
pub trait ParamMeta: AsPaths {
    // @todo/maybe For each concrete type store the set of keys that are style=deepObject vs style=form
    // - Allow application code to create logic based on these sets.
    // fn get_form_keys(&self) -> Vec<&str>;
    // fn get_deep_object_keys(&self) -> Vec<&str>;

    fn to_query_kv(&self) -> Vec<(String, String)> {
        flatten_paths(self.as_paths())
    }

    // Note: This can be done with reqwests "client.get(url).query".
    // @see https://docs.rs/reqwest/0.8.6/src/reqwest/request.rs.html#243-254
    // Alternative: https://github.com/nox/serde_urlencoded + https://github.com/samscott89/serde_qs (adds nesting).
    // fn to_query(&self) -> String {
    //     let kv = &self.to_query_kv();
    //
    //     let mut url = Url::parse("https://example.net").unwrap();
    //
    //     {
    //         let mut qp = url.query_pairs_mut();
    //
    //         for (k, v) in kv.iter() {
    //             // @todo/low `[` url encodes to `%5B` - its standards compliant but hard to read, replace these back to ascii?
    //             qp.append_pair(k, v);
    //         }
    //     }
    //
    //     url.query().unwrap().into()
    // }
}

impl<T> ParamMeta for T where T: AsPaths {}

// impl ParamMeta for S1 {
//     fn get_form_keys(&self) -> Vec<&str> {
//         vec!["f1", "f2"]
//     }
//     fn get_deep_object_keys(&self) -> Vec<&str> {
//         vec!["o1", "o2"]
//     }
// }
