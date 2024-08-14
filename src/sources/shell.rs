use std::{fs, path::Path};

use anyhow::anyhow;

use crate::{
  runnables::{AddRunnables, RunRunnable},
  types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct ShellParams {}

pub struct Shell;

impl AddRunnables for Shell {
  fn add_runnable(path: &Path, runnables: &mut Vec<Runnable>) -> anyhow::Result<()> {
    let metadata = path.metadata()?;
    if !metadata.is_dir() {
      return Err(anyhow!("path is not directory"));
    }
    let children = fs::read_dir(path)?;
    for child in children {
      let Ok(child) = child else {
        continue;
      };
      if let Ok(metadata) = child.metadata() {
        if !metadata.is_file() {
          continue;
        }
      }
      if child
        .path()
        .extension()
        .map(|extension| extension == "sh")
        .unwrap_or(false)
      {
        runnables.push(Runnable {
          name: child.path().display().to_string(),
          display_name: None,
          path: child.path(),
          index: 0,
          params: RunnableParams::Shell(ShellParams {}),
          description: Default::default(),
        })
      }
    }
    Ok(())
  }
}

impl RunRunnable for Shell {
  type Params = ShellParams;

  fn command(runnable: &Runnable, _: &Self::Params) -> String {
    format!("sh {}", runnable.path.display())
  }
}
