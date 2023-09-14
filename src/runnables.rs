use std::{fs, path::Path};

use anyhow::Context;
use run_command::run_command_pipe_to_terminal;

use crate::types::Runnable;

const IGNORE: [&str; 2] = ["target", "node_modules"];

pub fn ignore_dir(path: &Path) -> bool {
    IGNORE.iter().any(|ignore| path.ends_with(ignore))
}

pub trait FindRunnables {
    fn find_runnable(path: &Path) -> anyhow::Result<Vec<Runnable>>;

    fn find_runnables(path: &Path) -> Vec<Runnable> {
        let mut runnables = Vec::<Runnable>::new();
        if let Ok(_runnables) = Self::find_runnable(path) {
            runnables.extend(_runnables);
        }
        let entries = fs::read_dir(path).context(format!("failed to read path: {path:?}"));
        if entries.is_err() {
            return runnables;
        }
        for entry in entries.unwrap().flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let path = entry.path();
                    if !ignore_dir(&path) {
                        runnables.extend(Self::find_runnables(&path));
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
