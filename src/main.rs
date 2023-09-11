#[macro_use]
extern crate log;

use std::{fmt::Display, path::PathBuf, str::FromStr};

use clap::Parser;
use helpers::{absolute_path, runnable_path_display};
// use cursive::{
//     align::HAlign,
//     theme::{BorderStyle, Palette, StyleType, Theme},
//     views::{Button, Dialog, LinearLayout, TextView},
//     With,
// };

mod helpers;
mod ignore;
mod languages;
mod types;

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
    let root_path = absolute_path(&args.path)?.display().to_string();

    let rust_runnables = languages::rust::get_runnables(&path);
    let root_path = format!("{root_path}/");

    // let mut siv = make_cursive_app();

    // let mut runnables = LinearLayout::vertical();
    //
    // for runnable in rust_runnables {
    //     let text = format!(
    //         "[{}] -> {} -> ( {} )",
    //         runnable.rtype,
    //         runnable.name,
    //         runnable_path_display(&root_path, &runnable.path)?
    //     );
    //     runnables.add_child(Button::new(text.clone(), move |s| {
    //         let root_layer = s.pop_layer().unwrap();
    //         let dialog =
    //             Dialog::new()
    //                 .title(&text)
    //                 .button("run", |_s| {})
    //                 .button("back", move |s| {
    //                     s.pop_layer();
    //                     // s.add_layer(root_layer);
    //                 });
    //         s.add_layer(dialog);
    //     }));
    // }
    //
    // let dialog = Dialog::around(runnables)
    //     .title(format!(
    //         "runnables -> ( {} )",
    //         absolute_path(&args.path)?.display()
    //     ))
    //     .title_position(HAlign::Center)
    //     .padding_lrtb(2, 2, 2, 2);
    //
    // siv.add_layer(dialog);
    //
    // siv.run();

    Ok(())
}
