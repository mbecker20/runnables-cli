#[macro_use]
extern crate log;

use std::{path::PathBuf, str::FromStr};

use clap::Parser;

mod helpers;
mod ignore;
mod languages;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct RunnablesCli {
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() -> anyhow::Result<()> {
    helpers::init_logger(log::LevelFilter::Info)?;

    let args = RunnablesCli::parse();
    let path = PathBuf::from_str(&args.path)?;
    let paths = languages::rust::get_rust_runnables(&path);
    
    info!("{paths:#?}");
    Ok(())
}
