use std::{
  fs,
  path::{Path, PathBuf},
  rc::Rc,
};

use derive_variants::ExtractVariant;
use run_command::run_command_pipe_to_terminal;

use crate::{
  sources::run_runnable,
  types::{Runnable, RunnableParamsVariant},
};

const IGNORE: [&str; 3] = ["target", "node_modules", ".git"];

pub fn ignore_dir(path: &Path) -> bool {
  IGNORE.iter().any(|ignore| path.ends_with(ignore))
}

pub trait AddRunnables {
  fn add_runnable(path: &Path, runnables: &mut Vec<Runnable>) -> anyhow::Result<()>;

  fn add_runnables(
    path: &Path,
    runincludes: &[PathBuf],
    runignores: &[PathBuf],
    runnables: &mut Vec<Runnable>,
  ) {
    Self::add_runnable(path, runnables).ok();
    let Ok(entries) = fs::read_dir(path) else {
      return;
    };
    for entry in entries.flatten() {
      if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
          let path = entry.path();
          // Unwrap ok, path definitely on the system
          let norm = path.canonicalize().unwrap();
          if (!runincludes.is_empty() && runincludes.contains(&norm))
            && !ignore_dir(&path)
            && !runignores.contains(&norm)
          {
            Self::add_runnables(&path, runincludes, runignores, runnables);
          }
        }
      }
    }
  }
}

pub trait RunRunnable {
  type Params;

  fn command(runnable: &Runnable, params: &Self::Params) -> String;

  fn run(runnable: &Runnable, params: &Self::Params, runnables: &[Rc<Runnable>]) {
    if let Some(after) = &runnable.after {
      for after in after {
        let (runnable_type, after) = after
          .split_once(':')
          .map(|(ty, after)| {
            (
              ty.parse::<RunnableParamsVariant>()
                .expect("Invalid runnable variant in 'after'"),
              after,
            )
          })
          .unwrap_or((RunnableParamsVariant::RunFile, after));
        if let Some(before) = runnables
          .iter()
          .find(|r| r.params.extract_variant() == runnable_type && r.name == *after)
        {
          run_runnable(before, runnables);
        }
      }
    }
    let command = Self::command(runnable, params);
    run_command_pipe_to_terminal(&command);
  }
}
