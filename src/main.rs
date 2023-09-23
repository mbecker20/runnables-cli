use clap::Parser;
use helpers::wait_for_enter;
use ratatui::style::Color;

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
    #[arg(short, long)]
    search: bool,
    #[arg(short, long, default_value_t = Color::Blue)]
    color: Color,
}

fn main() -> anyhow::Result<()> {
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
