use serde_json::json;
use static_toml::static_toml;
use std::{fs, io};

static_toml! {
    #[derive(Debug, ::serde::Serialize)]
    #[static_toml(prefer_slices = false)]
    static CARGO_TOML = include_toml!("Cargo.toml");
}

fn main() -> io::Result<()> {
    build_jsr_json()?;
    Ok(())
}

fn build_jsr_json() -> io::Result<()> {
    let package = json!({
        "name": CARGO_TOML.package.name,
        "version": CARGO_TOML.package.version,
        "author": CARGO_TOML.package.authors.0,
        "license": CARGO_TOML.package.license,
        "exports": CARGO_TOML.package.metadata.jsr.exports,
        "publish": CARGO_TOML.package.metadata.jsr.publish,
        "repository": {
            "type": "git",
            "url": CARGO_TOML.package.repository
        }
    });

    let json = serde_json::to_string_pretty(&package).expect("valid json");
    fs::write("jsr.json", json)
}
