use std::{path::PathBuf, fmt::Display};

pub struct Runnable {
    pub name: String,
    pub rtype: RunnableType,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum RunnableType {
    Rust,
    Javascript,
}

impl Display for RunnableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}
