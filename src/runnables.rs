use std::{
  fs,
  path::{Path, PathBuf},
};

use run_command::run_command_pipe_to_terminal;

use crate::types::Runnable;

const IGNORE: [&str; 3] = ["target", "node_modules", ".git"];

pub fn ignore_dir(path: &Path) -> bool {
  IGNORE.iter().any(|ignore| path.ends_with(ignore))
}

pub trait FindRunnables {
  fn find_runnable(path: &Path) -> anyhow::Result<Vec<Runnable>>;

  fn find_runnables(path: &Path, runignores: &[PathBuf]) -> Vec<Runnable> {
    let mut runnables = Vec::<Runnable>::new();
    if let Ok(_runnables) = Self::find_runnable(path) {
      runnables.extend(_runnables);
    }
    let entries = fs::read_dir(path);
    if entries.is_err() {
      return runnables;
    }
    for entry in entries.unwrap().flatten() {
      if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
          let path = entry.path();
          let norm = path.canonicalize().unwrap();
          if !ignore_dir(&path) && !runignores.contains(&norm) {
            runnables.extend(Self::find_runnables(&path, runignores));
          }
        }
      }
    }
    runnables
  }
}

pub trait RunRunnable {
  type Params;

  fn command(runnable: &Runnable, params: &Self::Params) -> String;

  fn run(runnable: &Runnable, params: &Self::Params) {
    let command = Self::command(runnable, params);
    run_command_pipe_to_terminal(&command);
  }
}
