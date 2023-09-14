use std::path::Path;

use crate::{
    runnables::{FindRunnables, RunRunnable},
    types::{Runnable, RunnableParams},
};

use self::{runfile::RunFile, rust::Rust};

pub mod runfile;
pub mod rust;

pub fn get_runnables(path: &Path) -> Vec<Runnable> {
    let mut runnables = Vec::new();

    runnables.extend(RunFile::find_runnables(path));
    runnables.extend(Rust::find_runnables(path));

    runnables
}

pub fn run_runnable(runnable: Runnable) {
    match &runnable.params {
        RunnableParams::RunFile(params) => RunFile::run(&runnable, params),
        RunnableParams::Rust(params) => Rust::run(&runnable, params),
        // RunnableParams::Javascript() => todo!(),
        RunnableParams::None => {
            println!("got NONE runnable")
        }
    }
}
