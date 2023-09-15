use std::io::Read;

use anyhow::Context;
use clap::Parser;

use crate::{sources::run_runnable, types::RunnableParams};

mod components;
mod helpers;
mod runnables;
mod sources;
mod state;
mod tui;
mod types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() -> anyhow::Result<()> {
    // let root_path_display = absolute_path(&path)?.display().to_string();
    let mut state = state::State::new()?;

    if state.runnables.is_empty() {
        println!("no runnables found 🧐");
        wait_for_enter()?;
        return Ok(());
    }

    match tui::run(&mut state) {
        Ok(_) => {
            if let RunnableParams::None = state.runnable.params {
                // no selection was made
                return Ok(());
            }
            state.runnable.log_info();
            run_runnable(state.runnable);
        }
        Err(e) => {
            println!("\n{e:#?}");
        }
    }

    wait_for_enter()?;

    Ok(())
}

fn wait_for_enter() -> anyhow::Result<()> {
    println!("\nPress ENTER to close");
    let buffer = &mut [0u8];
    std::io::stdin()
        .read_exact(buffer)
        .context("failed to read ENTER")?;
    Ok(())
}
