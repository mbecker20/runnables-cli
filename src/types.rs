use std::{fmt::Display, path::PathBuf};

use clap::ValueEnum;
use colored::Colorize;
use derive_variants::EnumVariants;

use crate::sources::{
  javascript::JavascriptParams, runfile::RunFileParams, rust_bin::RustBinParams,
  rust_lib::RustLibParams, shell::ShellParams,
};

#[derive(Clone, Debug, Default)]
pub struct Runnable {
  pub name: String,
  pub display_name: Option<String>,
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
  RustBin(RustBinParams),
  RustLib(RustLibParams),
  Javascript(JavascriptParams),
}

impl Display for RunnableParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let d = match self {
      RunnableParams::RunFile(_) => "runfile",
      RunnableParams::Shell(_) => "shell",
      RunnableParams::RustBin(_) => "rust (bin)",
      RunnableParams::RustLib(_) => "rust (lib)",
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
      RunnableParams::RustBin(_) => RunnableParamsVariant::RustBin,
      RunnableParams::RustLib(_) => RunnableParamsVariant::RustLib,
      RunnableParams::RunFile(_) => RunnableParamsVariant::RunFile,
      RunnableParams::Shell(_) => RunnableParamsVariant::Shell,
      RunnableParams::Javascript(_) => RunnableParamsVariant::Javascript,
    }
  }
}

impl Runnable {
  pub fn log_info(&self) {
    println!("-----------------------");
    println!("running: {}", self.name.bright_blue());
    println!("type: {}", self.params.to_string().bright_blue());

    if let RunnableParams::RustBin(params) = &self.params {
      println!("command: {}", params.command.to_string().bright_blue());
    }

    println!("path: {}", self.path.display().to_string().bright_blue());
    println!("-----------------------\n");
  }
}
