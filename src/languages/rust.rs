use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use serde::Deserialize;

use crate::{
    ignore::ignore_dir,
    types::{Runnable, RunnableType},
};

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

pub fn get_runnables(path: &PathBuf) -> Vec<Runnable> {
    let mut runnables = Vec::<Runnable>::new();
    if let Ok(runnable) = get_runnable(path) {
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
                        runnables.extend(get_runnables(&path));
                    }
                }
            }
        }
    }
    runnables
}

fn get_runnable(path: &PathBuf) -> anyhow::Result<Runnable> {
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
    Ok(Runnable {
        name: cargo_toml.package.name,
        rtype: RunnableType::Rust,
        path: path.clone(),
    })
}

pub fn run_rust(runnable: &Runnable, release: bool, args: Option<&str>) {
    let command = run_rust_command(runnable, release, args);
    println!("{command}")
}

fn run_rust_command(runnable: &Runnable, release: bool, args: Option<&str>) -> String {
    let release = if release { " --release" } else { "" };
    let args = match args {
        Some(args) => format!(" -- {args}"),
        None => String::new(),
    };
    format!(
        "cd {} && cargo run {release}{args}",
        runnable.path.display()
    )
}

#[cfg(test)]
mod rust_tests {
    use super::*;

    // #[test]
    // fn rust_command_debug_no_args() {
    //     let result = run_rust_command("testo", false, None);
    //     assert_eq!(result, String::from("cargo run -p testo"));
    // }
    //
    // #[test]
    // fn rust_command_release_no_args() {
    //     let result = run_rust_command("testo", true, None);
    //     assert_eq!(result, String::from("cargo run -p testo --release"));
    // }
    //
    // #[test]
    // fn rust_command_debug_args() {
    //     let result = run_rust_command("testo", false, Some("--arg smth"));
    //     assert_eq!(result, String::from("cargo run -p testo -- --arg smth"));
    // }
    //
    // #[test]
    // fn rust_command_release_args() {
    //     let result = run_rust_command("testo", true, Some("--arg smth"));
    //     assert_eq!(
    //         result,
    //         String::from("cargo run -p testo --release -- --arg smth")
    //     );
    // }
}
