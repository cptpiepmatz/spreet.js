use serde_json::json;
use static_toml::static_toml;
use std::{fs, io};

static_toml! {
    #[derive(Debug)]
    #[static_toml(prefer_slices = false)]
    static CARGO_TOML = include_toml!("Cargo.toml");
}

fn main() -> io::Result<()> {
    fs::create_dir_all("dist")?;
    copy_wasm()?;
    copy_license()?;
    copy_plugin_ts()?;
    build_deno_json()?;
    Ok(())
}

fn copy_wasm() -> io::Result<()> {
    fs::copy(
        "target/wasm32-wasip1/release/esbuild_plugin_spreet.wasm",
        "dist/esbuild_plugin_spreet.wasm",
    )?;

    Ok(())
}

fn copy_license() -> io::Result<()> {
    fs::copy("LICENSE", "dist/LICENSE")?;

    Ok(())
}

fn copy_plugin_ts() -> io::Result<()> {
    fs::copy("src/plugin.ts", "dist/plugin.ts")?;

    Ok(())
}

fn build_deno_json() -> io::Result<()> {
    let name = format!(
        "{}/{}",
        CARGO_TOML.package.metadata.deno.scope, CARGO_TOML.package.name
    );
    let package = json!({
        "name": name,
        "version": CARGO_TOML.package.version,
        "author": CARGO_TOML.package.authors.0,
        "license": CARGO_TOML.package.license,
        "exports": "./plugin.ts",
        "publish": {
            "include": [
                "plugin.ts",
                "esbuild_plugin_spreet.wasm",
                "LICENSE"
            ]
        },
        "repository": {
            "type": "git",
            "url": CARGO_TOML.package.repository
        }
    });

    let json = serde_json::to_string_pretty(&package).expect("valid json");
    fs::write("dist/deno.json", json)
}
