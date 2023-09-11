use std::{collections::HashMap, path::Path};

use serde::Deserialize;

pub type RunFile = HashMap<String, RunFileItem>;

#[derive(Debug, Clone, Deserialize)]
pub struct RunFileItem {
    pub cmd: String,
    #[serde(default = "default_path")]
    pub path: String,
}

fn default_path() -> String {
    String::from(".")
}

pub fn get_runnables(path: &Path) {
    
}
