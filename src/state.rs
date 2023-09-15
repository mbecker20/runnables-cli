use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::{
    helpers::absolute_path,
    sources::{
        get_runnables,
        rust::{RustCommand, RustRunnableParams},
    },
    types::{Runnable, RunnableParams, RunnableParamsVariant},
    CliArgs,
};

pub struct State {
    pub args: CliArgs,
    pub runnables: Vec<Runnable>,
    pub selected: usize,
    pub runnable: Runnable,
}

impl State {
    pub fn new() -> anyhow::Result<State> {
        let args = CliArgs::parse();
        let mut runnables = get_runnables(&PathBuf::from_str(&args.path)?);
        runnables
            .iter_mut()
            .enumerate()
            .for_each(|(index, runnable)| {
                runnable.index = index;
            });
        let state = State {
            runnables,
            args,
            selected: 0,
            runnable: Default::default(),
        };
        Ok(state)
    }

    pub fn get_runnables_variants(&self, variant: RunnableParamsVariant) -> Vec<&Runnable> {
        self.runnables
            .iter()
            .filter(|runnable| {
                let var: RunnableParamsVariant = (&runnable.params).into();
                var == variant
            })
            .collect()
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
        self.selected %= self.runnables.len();
    }

    pub fn set_runnable(&mut self) {
        self.runnable = self.runnables[self.selected].clone();
    }

    /// returns true if should break render loop
    pub fn handle_keypress(&mut self, key: char) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::RunFile(_) => match key {
                'r' => {
                    self.set_runnable();
                    true
                }
                _ => false,
            },
            RunnableParams::Rust(_) => {
                let command = match key {
                    'r' => Some(RustCommand::Run),
                    'R' => Some(RustCommand::RunRelease),
                    't' => Some(RustCommand::Test),
                    'f' => Some(RustCommand::Fmt),
                    'c' => Some(RustCommand::Check),
                    'C' => Some(RustCommand::Clippy),
                    'b' => Some(RustCommand::Build),
                    'B' => Some(RustCommand::BuildRelease),
                    _ => None,
                };
                if let Some(command) = command {
                    self.set_runnable();
                    self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                        command,
                        args: None,
                    });
                    true
                } else {
                    false
                }
            }
            RunnableParams::None => todo!(),
        }
    }

    pub fn root_absolute_path(&self) -> anyhow::Result<String> {
        let path = absolute_path(&self.args.path)?.display().to_string();
        Ok(path)
    }
}
