use std::time::Instant;

use anyhow::anyhow;
use clap::Parser;
use colored::Colorize;
use derive_variants::ExtractVariant;
use helpers::wait_for_enter;
use ratatui::style::Color;
use types::RunnableParamsVariant;

use crate::{sources::run_runnable, types::RunnableParams};

mod helpers;
mod runnables;
mod sources;
mod state;
mod tui;
mod types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
  /// The root path to search for runnables.
  /// Default: "."
  #[arg(default_value_t = String::from("."))]
  path: String,
  /// Specify a specific runnable to run.
  /// Example:
  /// - "just-ls" or "RunFile:just-ls" - match to a runnable declared in runfile.
  /// - "RustBin:runnables-cli" - match to a runnable picked up as a rust binary crate.
  #[arg(short, long)]
  runnable: Option<String>,
  /// Open the interface with a pre-filled search query.
  #[arg(short, long)]
  search: Option<String>,
  /// Specify the color theme to use.
  #[arg(short, long, default_value_t = Color::Blue)]
  color: Color,
  /// Ignore specific runnable types
  #[arg(short, long)]
  ignore: Vec<RunnableParamsVariant>,
}

fn main() -> anyhow::Result<()> {
  let mut state = state::State::new()?;

  if state.runnables.is_empty() {
    println!("no runnables found 🧐");
    wait_for_enter()?;
    return Ok(());
  }

  if let Some(label) = state.args.runnable {
    let (runnable_type, label) = match label.split_once(':') {
      Some((runnable_type, label)) => (runnable_type, label),
      None => ("RunFile", label.as_str()),
    };
    let runnable_type = runnable_type.parse::<RunnableParamsVariant>()?;
    let Some(runnable) = state
      .runnables
      .iter()
      .find(|r| r.params.extract_variant() == runnable_type && r.name == label)
      .map(|r| r.as_ref().clone())
    else {
      return Err(anyhow!("runnable not found: {}", label));
    };
    state.runnable = runnable;
  } else {
    // Open the interface to select runnable.
    if let Err(e) = tui::run(&mut state) {
      println!("\n{e:#?}");
      return Ok(());
    }
  }

  if let RunnableParams::None = state.runnable.params {
    // no selection was made
    return Ok(());
  }
  state.runnable.log_info();
  let timer = Instant::now();
  run_runnable(&state.runnable, &state.runnables);
  println!(
    "\n{} {}",
    "FINISHED in".dimmed(),
    format!("{:.2?}", timer.elapsed()).bold()
  );

  Ok(())
}
