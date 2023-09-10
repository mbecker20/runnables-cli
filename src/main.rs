#[macro_use]
extern crate log;

use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use cursive::{
    align::HAlign,
    theme::{BorderStyle, StyleType, Theme, Palette},
    views::{Button, Dialog, LinearLayout, TextView}, With,
};

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
    let rust_runnables = languages::rust::get_runnables(&path);

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;

            {
                // First, override some colors from the base palette.
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }

            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
            }
        }),
    });

    let mut runnables = LinearLayout::vertical();

    runnables.add_child(TextView::new("select a runnable"));

    for runnable in rust_runnables {
        runnables.add_child(Button::new(&runnable, |_| {}));
    }

    let dialog = Dialog::around(runnables)
        .title("runnables")
        .title_position(HAlign::Center).padding_lrtb(10, 10, 10, 10);

    siv.add_layer(dialog);

    siv.run();

    Ok(())
}
