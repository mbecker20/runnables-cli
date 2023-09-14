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

    pub fn on_enter(&mut self) {
        self.runnable = self.runnables[self.selected].clone();
    }

    pub fn on_r(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::RunRelease,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    pub fn on_t(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::Test,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    pub fn on_f(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::Fmt,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    pub fn on_c(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::Check,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    pub fn on_b(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::Build,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    #[allow(non_snake_case)]
    pub fn on_B(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::BuildRelease,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }

    #[allow(non_snake_case)]
    pub fn on_C(&mut self) -> bool {
        match &self.runnables[self.selected].params {
            RunnableParams::Rust(_) => {
                self.on_enter();
                self.runnable.params = RunnableParams::Rust(RustRunnableParams {
                    command: RustCommand::Clippy,
                    args: None,
                });
                true
            }
            _ => false,
        }
    }
    pub fn root_absolute_path(&self) -> anyhow::Result<String> {
        let path = absolute_path(&self.args.path)?.display().to_string();
        Ok(path)
    }
}
