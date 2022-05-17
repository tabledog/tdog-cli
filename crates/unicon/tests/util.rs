use std::{fs, env};
use std::fs::File;
use uuid::Uuid;

pub fn get_temp_file(f: String) -> std::io::Result<String> {
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