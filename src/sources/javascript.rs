use std::{fmt::Display, fs, path::Path};

use anyhow::Context;
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
    runnables::{FindRunnables, RunRunnable},
    types::{Runnable, RunnableParams},
};

#[derive(Debug, Clone, Default)]
pub struct JavascriptRunnableParams {
    pub command: JavascriptCommand,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum JavascriptCommand {
    #[default]
    Yarn,
    Npm,
}

impl Display for JavascriptCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            JavascriptCommand::Yarn => "yarn",
            JavascriptCommand::Npm => "npm run",
        };
        f.write_str(d)
    }
}

#[derive(Deserialize)]
struct PackageJson {
    // name: String,
    scripts: IndexMap<String, String>,
}

pub struct Javascript;

impl FindRunnables for Javascript {
    fn find_runnable(path: &Path) -> anyhow::Result<Vec<crate::types::Runnable>> {
        let package_json_contents = fs::read_to_string(path.join("package.json"))
            .context("directory does not inclue package.json")?;
        let PackageJson { scripts } =
            serde_json::from_str(&package_json_contents).context("failed to parse package.json")?;
        let runnables = scripts
            .into_iter()
            .map(|(script_name, command)| Runnable {
                name: script_name,
                description: Some(command),
                path: path.to_owned(),
                index: 0,
                params: RunnableParams::Javascript(Default::default()),
            })
            .collect();
        Ok(runnables)
    }
}

impl RunRunnable for Javascript {
    type Params = JavascriptRunnableParams;

    fn command(runnable: &Runnable, params: &Self::Params) -> String {
        format!(
            "cd {} && {} {}",
            runnable.path.display(),
            params.command,
            runnable.name
        )
    }
}
