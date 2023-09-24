use std::{
    path::PathBuf,
    str::FromStr,
};

use crate::{
    runnables::{FindRunnables, RunRunnable},
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

    let mut runnables = Vec::new();

    if !args.ignore.contains(&RunnableParamsVariant::RunFile) {
        runnables.extend(RunFile::find_runnables(&path));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Shell) {
        runnables.extend(Shell::find_runnables(&path));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Rust) {
        runnables.extend(Rust::find_runnables(&path));
    }
    if !args.ignore.contains(&RunnableParamsVariant::Javascript) {
        runnables.extend(Javascript::find_runnables(&path));
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
