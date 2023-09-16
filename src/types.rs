use std::{fmt::Display, path::PathBuf};

use derive_variants::EnumVariants;

use crate::sources::{
    javascript::JavascriptRunnableParams, runfile::RunFileParams,
    rust::RustRunnableParams,
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
#[variant_derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnableParams {
    #[default]
    None,
    RunFile(RunFileParams),
    Rust(RustRunnableParams),
    Javascript(JavascriptRunnableParams),
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
            RunnableParams::Javascript(_) => {
                RunnableParamsVariant::Javascript
            }
        }
    }
}

impl Runnable {
    pub fn log_info(&self) {
        match &self.params {
            RunnableParams::Rust(params) => {
                println!("running: {}\ntype: {}\ncommand: {}\npath: {:?}\n", self.name, self.params, params.command, self.path);
            }
            _ => {
                println!(
                    "running: {}\ntype: {}\npath: {:?}\n",
                    self.name, self.params, self.path
                );
            }
        }
    }
}
