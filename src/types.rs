use std::{fmt::Display, path::PathBuf};

use derive_variants::EnumVariants;
use colored::Colorize;
use clap::ValueEnum;

use crate::sources::{
    javascript::JavascriptParams, runfile::RunFileParams,
    rust::RustParams, shell::ShellParams,
};

#[derive(Clone, Debug, Default)]
pub struct Runnable {
    pub name: String,
    pub description: Option<String>,
    pub path: PathBuf,
    pub index: usize,
    pub params: RunnableParams,
}

#[derive(Debug, Clone, Default, EnumVariants)]
#[variant_derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RunnableParams {
    #[default]
    None,
    RunFile(RunFileParams),
    Shell(ShellParams),
    Rust(RustParams),
    Javascript(JavascriptParams),
}

impl Display for RunnableParams {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let d = match self {
            RunnableParams::Rust(params) => {
                if params.is_lib {
                    "rust (lib)"
                } else {
                    "rust (bin)"
                }
            }
            RunnableParams::RunFile(_) => "runfile",
            RunnableParams::Shell(_) => "shell",
            RunnableParams::Javascript(_) => "javascript",
            RunnableParams::None => "none",
        };
        f.write_str(d)
    }
}

impl From<&RunnableParams> for RunnableParamsVariant {
    fn from(params: &RunnableParams) -> Self {
        match params {
            RunnableParams::None => RunnableParamsVariant::None,
            RunnableParams::Rust(_) => RunnableParamsVariant::Rust,
            RunnableParams::RunFile(_) => {
                RunnableParamsVariant::RunFile
            }
            RunnableParams::Shell(_) => RunnableParamsVariant::Shell,
            RunnableParams::Javascript(_) => {
                RunnableParamsVariant::Javascript
            }
        }
    }
}

impl Runnable {
    pub fn log_info(&self) {
        println!("-----------------------");
        println!("running: {}", self.name.bright_blue());
        println!("type: {}", self.params.to_string().bright_blue());

        if let RunnableParams::Rust(params) = &self.params {
            println!(
                "command: {}",
                params.command.to_string().bright_blue()
            );
        }

        println!(
            "path: {}",
            self.path.display().to_string().bright_blue()
        );
        println!("-----------------------\n");
    }
}
