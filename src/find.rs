use std::{path::Path, fs};

use anyhow::Context;

use crate::{types::Runnable, ignore::ignore_dir};

pub trait FindRunnables {
    fn find_runnable(path: &Path) -> anyhow::Result<Runnable>;

    fn find_runnables(path: &Path) -> Vec<Runnable> {
        let mut runnables = Vec::<Runnable>::new();
        if let Ok(runnable) = Self::find_runnable(path) {
            runnables.push(runnable);
        }
        let entries = fs::read_dir(path).context(format!("failed to read path: {path:?}"));
        if entries.is_err() {
            return Vec::new();
        }
        for entry in entries.unwrap() {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let path = entry.path();
                        if !ignore_dir(&path) {
                            runnables.extend(Self::find_runnables(&path));
                        }
                    }
                }
            }
        }
        runnables
    }
}
