use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    runnables::{FindRunnables, RunRunnable, ignore_dir},
    types::{Runnable, RunnableParams, RunnableParamsVariant},
    CliArgs,
};

use self::{
    javascript::Javascript, runfile::RunFile, rust::Rust,
    shell::Shell,
};

pub mod javascript;
pub mod runfile;
pub mod rust;
pub mod shell;

pub fn get_runnables(
    args: &CliArgs,
) -> anyhow::Result<Vec<Runnable>> {
    let path = PathBuf::from_str(&args.path)?;

    let runignores = get_runignores(&path);

    let mut runnables = Vec::new();

    if !args.ignore.contains(&RunnableParamsVariant::RunFile) {
        runnables.extend(RunFile::find_runnables(&path, &runignores));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Shell) {
        runnables.extend(Shell::find_runnables(&path, &runignores));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Rust) {
        runnables.extend(Rust::find_runnables(&path, &runignores));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Javascript) {
        runnables.extend(Javascript::find_runnables(&path, &runignores));
    }

    Ok(runnables)
}

pub fn run_runnable(runnable: Runnable) {
    match &runnable.params {
        RunnableParams::RunFile(params) => {
            RunFile::run(&runnable, params)
        }
        RunnableParams::Rust(params) => Rust::run(&runnable, params),
        RunnableParams::Shell(params) => {
            Shell::run(&runnable, params)
        }
        RunnableParams::Javascript(params) => {
            Javascript::run(&runnable, params)
        }
        RunnableParams::None => {
            println!("got NONE runnable")
        }
    }
}

fn get_runignores(path: &Path) -> Vec<PathBuf> {
    let mut runignores = Vec::<PathBuf>::new();
    runignores.extend(get_runignore(path));
    let entries = fs::read_dir(path);
    if entries.is_err() {
        return runignores;
    }
    for entry in entries.unwrap().flatten() {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir() {
                let path = entry.path();
                if !ignore_dir(&path) {
                    runignores.extend(get_runignores(&path));
                }
            }
        }
    }
    runignores
}

fn get_runignore(path: &Path) -> Vec<PathBuf> {
    let runignore = fs::read_to_string(path.join(".runignore"));
    if runignore.is_err() {
        return Vec::new();
    }
    runignore
        .unwrap()
        .split('\n')
        .flat_map(|p| path.join(p).canonicalize())
        .collect()
}
