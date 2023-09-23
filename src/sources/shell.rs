use std::{fs, path::Path};

use anyhow::anyhow;

use crate::{
    runnables::{FindRunnables, RunRunnable},
    types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct ShellParams {}

pub struct Shell;

impl FindRunnables for Shell {
    fn find_runnable(path: &Path) -> anyhow::Result<Vec<Runnable>> {
        let metadata = path.metadata()?;
        if !metadata.is_dir() {
            return Err(anyhow!("path is not directory"));
        }
        let children = fs::read_dir(path)?;
        let mut runnables: Vec<Runnable> = Default::default();
        for child in children {
            if child.is_err() {
                continue;
            }
            let child = child.unwrap();
            if let Ok(metadata) = child.metadata() {
                if !metadata.is_file() {
                    continue;
                }
            }
            if child.path().extension().map(|extension| extension == "sh").unwrap_or(false) {
                runnables.push(Runnable {
                    name: child.path().display().to_string(),
                    path: child.path(),
                    index: 0,
                    params: RunnableParams::Shell(ShellParams {}),
                    description: Default::default(),
                })
            }
        }
        Ok(runnables)
    }
}

impl RunRunnable for Shell {
    type Params = ShellParams;

    fn command(runnable: &Runnable, _: &Self::Params) -> String {
        format!("sh {}", runnable.path.display())
    }
}
