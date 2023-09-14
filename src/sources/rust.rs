use std::{fmt::Display, fs, path::Path};

use anyhow::{anyhow, Context};
use serde::Deserialize;

use crate::{
    runnables::{FindRunnables, RunRunnable},
    types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct RustRunnableParams {
    pub command: RustCommand,
    pub args: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RustCommand {
    #[default]
    RunDebug,
    RunRelease,
    Build,
    BuildRelease,
    Test,
    Fmt,
    Check,
    Clippy,
}

impl Display for RustCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            RustCommand::RunDebug => "cargo run",
            RustCommand::RunRelease => "cargo run --release",
            RustCommand::Test => "cargo test",
            RustCommand::Fmt => "cargo fmt",
            RustCommand::Check => "cargo check",
            RustCommand::Clippy => "cargo clippy",
            RustCommand::Build => "cargo build",
            RustCommand::BuildRelease => "cargo build --release",
        };
        f.write_str(d)
    }
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
    description: Option<String>,
}

pub struct Rust;

impl FindRunnables for Rust {
    fn find_runnable(path: &Path) -> anyhow::Result<Vec<Runnable>> {
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
        Ok(vec![Runnable {
            name: cargo_toml.package.name,
            description: cargo_toml.package.description,
            path: path.to_owned(),
            index: 0,
            params: RunnableParams::Rust(Default::default()),
        }])
    }
}

impl RunRunnable for Rust {
    type Params = RustRunnableParams;

    fn command(runnable: &Runnable, params: &Self::Params) -> String {
        let args = match &params.args {
            Some(args) => format!(" -- {args}"),
            None => String::new(),
        };
        format!("cd {} && {}{args}", runnable.path.display(), params.command)
    }
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
