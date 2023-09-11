use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use serde::Deserialize;

use crate::{
    find::FindRunnables,
    types::{Runnable, RunnableParamsVariant},
};

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

pub struct Rust;

impl FindRunnables for Rust {
    fn find_runnable(path: &Path) -> anyhow::Result<Runnable> {
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
            path: path.to_owned(),
            rtype: RunnableParamsVariant::Rust,
        })
    }
}

pub fn run_rust_command(
    runnable: &Runnable,
    release: bool,
    test: bool,
    args: &Option<String>,
) -> String {
    if test {
        return format!(
            "cd {} && cargo test",
            runnable.path.display()
        );
    }
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
    // use super::*;

    #[test]
    fn run_test() {
        assert_eq!("it works", "it works")
    }

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
