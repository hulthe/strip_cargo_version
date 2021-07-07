use crate::Error;
use std::fs::{read_to_string, write};
use toml_edit::{value, Document};

const LOCK_FILE: &str = "Cargo.lock";

pub fn strip_lockfile(packages: &[&str]) -> Result<(), Error> {
    let lockfile = read_to_string(LOCK_FILE)?;
    let mut doc = lockfile.parse::<Document>().expect("invalid doc");

    let packages_toml = doc["package"]
        .as_array_of_tables_mut()
        .expect("lockfile: packages must be array of tables");

    let mut i = 0usize..;
    while let Some(package_toml) = packages_toml.get_mut(i.next().unwrap()) {
        let name = package_toml["name"]
            .as_value()
            .expect("package name must be a value")
            .to_string();

        if packages.contains(&name.as_str()) {
            package_toml["version"] = value("0.0.0");
        }
    }

    write(LOCK_FILE, doc.to_string())?;
    Ok(())
}
