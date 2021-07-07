mod cargo_lock;
mod cargo_toml;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let package_name = cargo_toml::strip_manifest()?;
    cargo_lock::strip_lockfile(&[&package_name])?;

    Ok(())
}
