use serde_json::json;
use static_toml::static_toml;
use std::{fs, io, path::{Path, PathBuf}};
use uu_cp::{self, Attributes, BackupMode, ClobberMode, CopyMode, Options, OverwriteMode, Preserve, ReflinkMode, SparseMode, UpdateMode};


static_toml! {
    #[derive(Debug, ::serde::Serialize)]
    static CARGO_TOML = include_toml!("Cargo.toml");
}

fn main() -> io::Result<()> {
    prepare_dist_dir()?;
    build_jsr_json()?;
    copy_files()?;
    Ok(())
}

fn prepare_dist_dir() -> io::Result<()> {
    match fs::remove_dir_all("dist") {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        err => err
    }?;

    fs::create_dir_all("dist")?;
    Ok(())
}

fn build_jsr_json() -> io::Result<()> {
    let package = json!({
        "name": CARGO_TOML.package.metadata.jsr.name,
        "version": CARGO_TOML.package.version,
        "author": CARGO_TOML.package.authors[0],
        "license": CARGO_TOML.package.license,
        "exports": CARGO_TOML.package.metadata.jsr.exports,
        "publish": CARGO_TOML.package.metadata.jsr.publish,
        "repository": {
            "type": "git",
            "url": CARGO_TOML.package.repository
        }
    });

    let json = serde_json::to_string_pretty(&package).expect("valid json");
    fs::write("dist/jsr.json", json)
}

fn copy_files() -> io::Result<()> {
    let dist = Path::new("dist");
    let sources: Vec<_> = CARGO_TOML.package.metadata.jsr.publish.include.iter().map(PathBuf::from).collect();
    
    let attributes = Attributes {
        #[cfg(unix)]
        ownership: Preserve::No { explicit: false },
        mode: Preserve::No { explicit: false },
        timestamps: Preserve::No { explicit: false },
        context: Preserve::No { explicit: false },
        links: Preserve::No { explicit: false },
        xattr: Preserve::No { explicit: false },
    };
    let options = Options {
        attributes_only: false, // Copy both file contents and attributes
        backup: BackupMode::NoBackup, // Do not create backups
        copy_contents: false, // Do not copy contents of special files
        cli_dereference: false, // Do not follow symlinks on the command line
        copy_mode: CopyMode::Copy, // Perform a standard copy
        dereference: false, // Copy symlinks as symlinks
        no_target_dir: false, // Allow copying multiple sources into the target directory
        one_file_system: false, // Allow copying across file systems
        overwrite: OverwriteMode::Clobber(ClobberMode::Force), // Overwrite existing files without prompt
        parents: false, // Do not create parent directories
        sparse_mode: SparseMode::Auto, // Automatically detect sparse files
        strip_trailing_slashes: false, // Do not strip trailing slashes from source arguments
        reflink_mode: ReflinkMode::Never,
        attributes,
        recursive: true, // Recursively copy directories
        backup_suffix: String::from("~"), // Default backup suffix
        target_dir: None, // No specific target directory
        update: UpdateMode::ReplaceAll, // Replace all files
        debug: false, // Disable debug output
        verbose: false, // Disable verbose output
        progress_bar: false, // Disable progress bar
    };
    
    uu_cp::copy(&sources, dist, &options).unwrap();
    Ok(())
}

