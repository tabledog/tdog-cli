use std::{env, fs};
use std::fs::File;

use uuid::Uuid;

pub(crate) fn get_temp_file(f: String) -> std::io::Result<String> {
    let mut dir = env::temp_dir();
    dir.push("td-data");
    fs::create_dir_all(dir.clone().into_os_string().into_string().unwrap().as_str())?;
    dir.push(f);
    let d2 = dir.clone();

    // Assert: is writable.
    let f = File::create(dir)?;
    f.sync_all()?;

    Ok(d2.canonicalize()?.into_os_string().into_string().unwrap())
}

pub fn get_unique_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

pub fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}


pub static REDACT_PLACEHOLDER: &'static str = "****redacted****";

// Hide parts of private keys so they can be logged/identified but not used.
pub trait Redact {
    // E.g: 123456789 => 12****redacted****89
    fn redact(&self, first: u32, last: u32) -> String;
}

impl Redact for &str {
    fn redact(&self, first: u32, last: u32) -> String {
        let hide_at_least = 10;
        if self.len() < (first + hide_at_least + last) as usize {
            panic!("Cannot redact information as the two sides overlap, which would leave 100% of the data visible in logs.")
        }

        format!("{}{}{}", &self[0..first as usize], REDACT_PLACEHOLDER, &self[self.len() - last as usize..])
    }
}



