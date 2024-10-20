use std::{fmt::Display, fs, path::Path};

use anyhow::anyhow;
use serde::Deserialize;

use crate::{
  runnables::{AddRunnables, RunRunnable},
  types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct RustBinParams {
  pub command: RustBinCommand,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RustBinCommand {
  #[default]
  Run,
  RunRelease,
  Publish,
  Install,
  Build,
  BuildRelease,
  Test,
  Fmt,
  Check,
  Clippy,
}

impl Display for RustBinCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let d = match self {
      RustBinCommand::Run => "cargo run",
      RustBinCommand::RunRelease => "cargo run --release",
      RustBinCommand::Publish => "cargo publish",
      RustBinCommand::Install => "cargo install --path .",
      RustBinCommand::Test => "cargo test",
      RustBinCommand::Fmt => "cargo fmt",
      RustBinCommand::Check => "cargo check",
      RustBinCommand::Clippy => "cargo clippy",
      RustBinCommand::Build => "cargo build",
      RustBinCommand::BuildRelease => "cargo build --release",
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

pub struct RustBin;

impl AddRunnables for RustBin {
  fn add_runnable(path: &Path, runnables: &mut Vec<Runnable>) -> anyhow::Result<()> {
    let metadata = path.metadata()?;
    if !metadata.is_dir() {
      return Err(anyhow!("path is not directory"));
    }
    let cargo_toml_contents = fs::read_to_string(path.join("Cargo.toml"))?;
    let CargoToml {
      package: CargoTomlPackage { name, description },
    } = toml::from_str(&cargo_toml_contents)?;

    if let Ok(bin) = fs::metadata(path.join("src/main.rs")) {
      if bin.is_file() {
        runnables.push(Runnable {
          name: name.clone(),
          display_name: None,
          description: description.clone(),
          after: None,
          path: path.to_owned(),
          index: 0,
          params: RunnableParams::RustBin(Default::default()),
        })
      }
    }

    Ok(())
  }
}

impl RunRunnable for RustBin {
  type Params = RustBinParams;

  fn command(runnable: &Runnable, params: &Self::Params) -> String {
    format!("cd {} && {}", runnable.path.display(), params.command)
  }
}

#[cfg(test)]
mod rust_tests {
  #[test]
  fn run_test() {
    assert_eq!("it works", "it works")
  }
}
