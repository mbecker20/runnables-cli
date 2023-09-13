use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{helpers::absolute_path, sources::get_runnables, types::Runnable, CliArgs};

pub struct State {
    pub args: CliArgs,
    pub runnables: Vec<Runnable>,
    pub selected: usize,
    pub runnable: Runnable,
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        let args = CliArgs::parse();
        let state = State {
            runnables: get_runnables(&PathBuf::from_str(&args.path)?),
            args,
            selected: 0,
            runnable: Default::default(),
        };
        Ok(state)
    }

    pub fn on_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.runnables.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn on_down(&mut self) {
        self.selected += 1;
        self.selected = self.selected % self.runnables.len();
    }

    pub fn on_enter(&mut self) {
        self.runnable = self.runnables[self.selected].clone();
    }

    pub fn root_absolute_path(&self) -> anyhow::Result<String> {
        let path = absolute_path(&self.args.path)?.display().to_string();
        Ok(path)
    }
}
