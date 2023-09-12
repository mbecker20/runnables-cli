use std::{fmt::Display, path::PathBuf};

use crate::sources::{runfile::RunFileParams, rust::RustRunnableParams};

#[derive(Clone, Debug, Default)]
pub struct Runnable {
    pub name: String,
    pub description: Option<String>,
    pub path: PathBuf,
    pub params: RunnableParams,
}

#[derive(Debug, Clone, Default)]
pub enum RunnableParams {
    #[default]
    None,
    Rust(RustRunnableParams),
    RunFile(RunFileParams),
    Javascript(),
}

impl Display for RunnableParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            RunnableParams::Rust(_) => "rust",
            RunnableParams::RunFile(_) => "runfile",
            RunnableParams::Javascript() => "js",
            RunnableParams::None => "none",
        };
        f.write_str(d)
    }
}

impl Runnable {
    pub fn log_info(&self) {
        println!(
            "running: {}\ntype: {}\npath: {:?}\n",
            self.name, self.params, self.path
        );
    }
}
