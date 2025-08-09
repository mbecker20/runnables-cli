use std::{fmt::Display, path::PathBuf};

use clap::ValueEnum;
use colored::Colorize;
use derive_variants::EnumVariants;
use strum::EnumString;

use crate::sources::{
  runfile::RunFileParams, rust_bin::RustBinParams, rust_lib::RustLibParams, shell::ShellParams,
};

#[derive(Clone, Debug, Default)]
pub struct Runnable {
  pub name: String,
  pub aliases: Vec<String>,
  pub display_name: Option<String>,
  pub description: Option<String>,
  /// The names of other runnables to run before this one, in order from first to last.
  pub after: Option<Vec<String>>,
  pub path: PathBuf,
  pub index: usize,
  pub params: RunnableParams,
}

#[derive(Debug, Clone, Default, EnumVariants)]
#[variant_derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, ValueEnum, EnumString
)]
pub enum RunnableParams {
  #[default]
  None,
  RunFile(RunFileParams),
  Shell(ShellParams),
  RustBin(RustBinParams),
  RustLib(RustLibParams),
}

impl Display for RunnableParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let d = match self {
      RunnableParams::RunFile(_) => "runfile",
      RunnableParams::Shell(_) => "shell",
      RunnableParams::RustBin(_) => "rust (bin)",
      RunnableParams::RustLib(_) => "rust (lib)",
      RunnableParams::None => "none",
    };
    f.write_str(d)
  }
}

impl Runnable {
  pub fn log_info(&self) {
    println!("-----------------------");
    println!("running: {}", self.name.bright_blue());
    println!("type: {}", self.params.to_string().bright_blue());

    if let Some(after) = &self.after {
      println!("after: {}", format!("{after:?}").bright_blue());
    }

    if let RunnableParams::RustBin(params) = &self.params {
      println!("command: {}", params.command.to_string().bright_blue());
    }

    println!("path: {}", self.path.display().to_string().bright_blue());
    println!("-----------------------\n");
  }

  pub fn aliases(&self) -> Vec<&str> {
    self.aliases.iter().map(String::as_str).collect::<Vec<_>>()
  }
}
