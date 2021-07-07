use crate::Error;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use toml_edit::{value, Document};

const MANIFEST_FILE: &str = "Cargo.toml";

pub struct Manifest {
    path: PathBuf,
    doc: Document,
}

impl Manifest {
    pub fn open<P: AsRef<Path>>(folder: P) -> Result<Self, Error> {
        let folder: &Path = folder.as_ref();
        let path = folder.join(MANIFEST_FILE);

        let manifest = read_to_string(&path)?;
        let doc = manifest.parse::<Document>()?;

        Ok(Manifest { path, doc })
    }

    pub fn strip_version(&mut self) {
        if self.doc.as_table().contains_table("package") {
            self.doc["package"]["version"] = value("0.0.0");
        }
    }

    pub fn get_package_name(&self) -> Option<String> {
        if self.doc.as_table().contains_table("package") {
            Some(
                self.doc["package"]["name"]
                    .as_value()
                    .expect("package name must be a value")
                    .to_string(),
            )
        } else {
            None
        }
    }

    pub fn get_members(&self) -> Vec<String> {
        if self.doc.as_table().contains_table("workspace") {
            self.doc["workspace"]["members"]
                .as_array()
                .expect("workspace members must be array")
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .expect("workspace member must be string")
                        .to_string()
                })
                .collect()
        } else {
            vec![]
        }
    }
}

impl Drop for Manifest {
    fn drop(&mut self) {
        write(&self.path, self.doc.to_string()).ok();
    }
}
