use serde_json::json;
use static_toml::static_toml;
use std::{fs, io};

static_toml! {
    #[derive(Debug)]
    static CARGO_TOML = include_toml!("Cargo.toml");
}

fn main() -> io::Result<()> {
    fs::create_dir_all("dist")?;
    copy_wasm()?;
    copy_plugin_js()?;
    build_package_json()?;
    Ok(())
}

fn copy_wasm() -> io::Result<()> {
    fs::copy(
        "target/wasm32-wasip1/release/esbuild_plugin_spreet.wasm",
        "dist/esbuild_plugin_spreet.wasm",
    )?;

    Ok(())
}

fn copy_plugin_js() -> io::Result<()> {
    fs::copy(
        "src/plugin.js",
        "dist/plugin.js",
    )?;
    
    Ok(())
}

fn build_package_json() -> io::Result<()> {
    let name = format!(
        "{}/{}",
        CARGO_TOML.package.metadata.package_json.scope, CARGO_TOML.package.name
    );
    let package = json!({
        "name": name,
        "version": CARGO_TOML.package.version,
        "main": "dist/plugin.js",
        "type": "module"
    });

    let json = serde_json::to_string_pretty(&package).expect("valid json");
    fs::write("dist/package.json", json)
}
