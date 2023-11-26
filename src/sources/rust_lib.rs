use std::{fmt::Display, fs, path::Path};

use anyhow::anyhow;
use serde::Deserialize;

use crate::{
  runnables::{FindRunnables, RunRunnable},
  types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct RustLibParams {
  pub command: RustLibCommand,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RustLibCommand {
  #[default]
  Publish,
  Build,
  BuildRelease,
  Test,
  Fmt,
  Check,
  Clippy,
}

impl Display for RustLibCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let d = match self {
      RustLibCommand::Publish => "cargo publish",
      RustLibCommand::Build => "cargo build",
      RustLibCommand::BuildRelease => "cargo build --release",
      RustLibCommand::Test => "cargo test",
      RustLibCommand::Fmt => "cargo fmt",
      RustLibCommand::Check => "cargo check",
      RustLibCommand::Clippy => "cargo clippy",
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

pub struct RustLib;

impl FindRunnables for RustLib {
  fn find_runnable(path: &Path) -> anyhow::Result<Vec<Runnable>> {
    let metadata = path.metadata()?;
    if !metadata.is_dir() {
      return Err(anyhow!("path is not directory"));
    }
    let cargo_toml_contents = fs::read_to_string(path.join("Cargo.toml"))?;
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
          params: RunnableParams::RustLib(RustLibParams::default()),
        })
      }
    }

    Ok(runnables)
  }
}

impl RunRunnable for RustLib {
  type Params = RustLibParams;

  fn command(runnable: &Runnable, params: &Self::Params) -> String {
    format!("cd {} && {}", runnable.path.display(), params.command)
  }
}
