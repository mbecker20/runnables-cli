use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use serde::Deserialize;

use crate::ignore::ignore_dir;

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

pub fn get_rust_runnables(path: &PathBuf) -> Vec<String> {
    let mut runnables = Vec::<String>::new();
    if let Ok(runnable) = check_rust_runnable_dir(path) {
        runnables.push(runnable);
    }
    let entries = fs::read_dir(path).context(format!("failed to read path: {path:?}"));
    if entries.is_err() {
        return Vec::new();
    }
    for entry in entries.unwrap() {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let path = entry.path();
                    if !ignore_dir(&path) {
                        let rs = get_rust_runnables(&path);
                        runnables.extend(rs);
                    }
                }
            }
        }
    }
    runnables
}

fn check_rust_runnable_dir(path: &PathBuf) -> anyhow::Result<String> {
    let metadata = path
        .metadata()
        .context(format!("could not get directory metadata: {path:?}"))?;
    if !metadata.is_dir() {
        return Err(anyhow!("path is not directory"));
    }
    let is_runnable = fs::metadata(path.join("src/main.rs"))
        .context(format!("could not find src/main.rs: {path:?}"))?
        .is_file();
    if !is_runnable {
        return Err(anyhow!("src/main.rs is not a file: {path:?}"));
    }
    let cargo_toml_contents = fs::read_to_string(path.join("Cargo.toml"))
        .context(format!("directory does not include Cargo.toml: {path:?}"))?;
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_contents)
        .context(format!("failed to parse Cargo.toml: {path:?}"))?;
    Ok(cargo_toml.package.name)
}
