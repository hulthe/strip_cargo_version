mod cargo_lock;
mod cargo_toml;

use cargo_toml::Manifest;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let root_manifest = Manifest::open(".")?;

    let manifests: Vec<_> = root_manifest
        .get_members()
        .into_iter()
        .map(Manifest::open)
        .chain([Ok(root_manifest)])
        .collect::<Result<_, _>>()?;

    for mut manifest in manifests {
        manifest.strip_version();
        if let Some(name) = manifest.get_package_name() {
            cargo_lock::strip_lockfile(&[&name])?;
        }
    }

    Ok(())
}
