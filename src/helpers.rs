use anyhow::Context;
use simple_logger::SimpleLogger;

pub fn init_logger(level: log::LevelFilter) -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_level(level)
        .env()
        .with_colors(true)
        .with_utc_timestamps()
        .init()
        .context("failed to init logger")
}
