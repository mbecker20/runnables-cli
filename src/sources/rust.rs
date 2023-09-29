use std::{fmt::Display, fs, path::Path};

use anyhow::anyhow;
use serde::Deserialize;

use crate::{
    runnables::{FindRunnables, RunRunnable},
    types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct RustParams {
    pub command: RustCommand,
    pub is_lib: bool,
    pub args: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RustCommand {
    #[default]
    Run,
    RunRelease,
    Install,
    Build,
    BuildRelease,
    Test,
    Fmt,
    Check,
    Clippy,
    Publish,
}

impl Display for RustCommand {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let d = match self {
            RustCommand::Run => "cargo run",
            RustCommand::RunRelease => "cargo run --release",
            RustCommand::Install => "cargo install --path .",
            RustCommand::Test => "cargo test",
            RustCommand::Fmt => "cargo fmt",
            RustCommand::Check => "cargo check",
            RustCommand::Clippy => "cargo clippy",
            RustCommand::Build => "cargo build",
            RustCommand::BuildRelease => "cargo build --release",
            RustCommand::Publish => "cargo publish",
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
        let metadata = path.metadata()?;
        if !metadata.is_dir() {
            return Err(anyhow!("path is not directory"));
        }
        let cargo_toml_contents =
            fs::read_to_string(path.join("Cargo.toml"))?;
        let CargoToml {
            package: CargoTomlPackage { name, description },
        } = toml::from_str(&cargo_toml_contents)?;

        let mut runnables: Vec<Runnable> = Default::default();

        if let Ok(lib) = fs::metadata(path.join("src/lib.rs")) {
            if lib.is_file() {
                runnables.push(Runnable {
                    name: name.clone(),
                    display_name: None,
                    description: description.clone(),
                    path: path.to_owned(),
                    index: 0,
                    params: RunnableParams::Rust(
                        RustParams {
                            is_lib: true,
                            ..Default::default()
                        },
                    ),
                })
            }
        }

        if let Ok(bin) = fs::metadata(path.join("src/main.rs")) {
            if bin.is_file() {
                runnables.push(Runnable {
                    name: name.clone(),
                    display_name: None,
                    description: description.clone(),
                    path: path.to_owned(),
                    index: 0,
                    params: RunnableParams::Rust(Default::default()),
                })
            }
        }

        Ok(runnables)
    }
}

impl RunRunnable for Rust {
    type Params = RustParams;

    fn command(runnable: &Runnable, params: &Self::Params) -> String {
        let args = match &params.args {
            Some(args) => format!(" -- {args}"),
            None => String::new(),
        };
        format!(
            "cd {} && {}{args}",
            runnable.path.display(),
            params.command
        )
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
