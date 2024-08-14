use std::{fs, path::Path};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
  runnables::{AddRunnables, RunRunnable},
  types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct RunFileParams {
  pub cmd: String,
}

pub type RunFileContent = IndexMap<String, RunFileItem>;

#[derive(Deserialize)]
pub struct RunFileItem {
  pub cmd: String,
  pub description: Option<String>,
  #[serde(default = "default_path")]
  pub path: String,
}

fn default_path() -> String {
  String::from(".")
}

pub struct RunFile;

impl AddRunnables for RunFile {
  fn add_runnable(path: &Path, runnables: &mut Vec<Runnable>) -> anyhow::Result<()> {
    let file_path = path.join("runfile.toml");
    let contents = fs::read_to_string(file_path)?;
    let contents: RunFileContent = toml::from_str(&contents)?;
    runnables.extend(contents.into_iter().map(|(name, item)| Runnable {
      name,
      display_name: None,
      description: item.description,
      path: path.join(item.path),
      index: 0,
      params: RunnableParams::RunFile(RunFileParams { cmd: item.cmd }),
    }));
    Ok(())
  }
}

impl RunRunnable for RunFile {
  type Params = RunFileParams;

  fn command(runnable: &Runnable, params: &Self::Params) -> String {
    format!("cd {} && {}", runnable.path.display(), params.cmd)
  }
}
