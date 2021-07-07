use crate::Error;
use std::fs::{read_to_string, write};
use toml_edit::{value, Document};

const MANIFEST_FILE: &str = "Cargo.toml";

pub fn strip_manifest() -> Result<String, Error> {
    let manifest = read_to_string(MANIFEST_FILE)?;
    let mut doc = manifest.parse::<Document>().expect("invalid doc");

    // strip version
    doc["package"]["version"] = value("0.0.0");

    let package_name = doc["package"]["name"]
        .as_value()
        .expect("package name must be a value")
        .to_string();

    write(MANIFEST_FILE, doc.to_string())?;

    Ok(package_name)
}
