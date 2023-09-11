use std::{cell::RefCell, io::Read, path::PathBuf, rc::Rc, str::FromStr};

use clap::Parser;
use cursive::{
    align::HAlign,
    views::{Button, Dialog, LinearLayout},
};
use helpers::{absolute_path, make_cursive_app, runnable_path_display};
use languages::rust::run_rust_command;
use run_command::run_command_pipe_to_terminal;
use types::{Runnable, RunnableParams};

use crate::{find::FindRunnables, languages::rust::Rust};

mod components;
mod find;
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

struct State {
    runnable: Runnable,
    params: RunnableParams,
}

fn main() -> anyhow::Result<()> {
    let args = RunnablesCli::parse();
    let path = PathBuf::from_str(&args.path)?;
    let root_path = absolute_path(&args.path)?.display().to_string();

    let rust_runnables = Rust::find_runnables(&path);
    let root_path = format!("{root_path}/");

    let mut siv = make_cursive_app();

    let state: Rc<RefCell<Option<State>>> = Default::default();

    let mut runnables = LinearLayout::vertical();

    for runnable in rust_runnables {
        let text = format!(
            " {:?} -> {} -> {} ",
            runnable.rtype,
            runnable.name,
            runnable_path_display(&root_path, &runnable.path)?
        );
        let state = state.clone();
        runnables.add_child(Button::new(text.clone(), move |s| {
            let mut dialog = Dialog::text(text.clone())
                .title("choose params")
                .padding_lrtb(2, 2, 2, 2);

            let _state = state.clone();
            let _runnable = runnable.clone();
            dialog.add_button("debug", move |s| {
                s.quit();
                _state.replace(Some(State {
                    runnable: _runnable.clone(),
                    params: RunnableParams::Rust {
                        release: false,
                        test: false,
                        args: None,
                    },
                }));
            });

            let _state = state.clone();
            let _runnable = runnable.clone();
            dialog.add_button("release", move |s| {
                s.quit();
                _state.replace(Some(State {
                    runnable: _runnable.clone(),
                    params: RunnableParams::Rust {
                        release: true,
                        test: false,
                        args: None,
                    },
                }));
            });

            let _state = state.clone();
            let _runnable = runnable.clone();
            dialog.add_button("test", move |s| {
                s.quit();
                _state.replace(Some(State {
                    runnable: _runnable.clone(),
                    params: RunnableParams::Rust {
                        release: false,
                        test: true,
                        args: None,
                    },
                }));
            });

            s.add_layer(dialog);
        }));
    }

    let dialog = Dialog::around(runnables)
        .title(format!(
            "runnables -> ( {} )",
            absolute_path(&args.path)?.display()
        ))
        .title_position(HAlign::Center)
        .padding_lrtb(2, 2, 2, 2);

    siv.add_layer(dialog);

    siv.run();

    let state = state.borrow();

    if state.is_none() {
        return Ok(());
    }

    let State { runnable, params } = state.as_ref().unwrap();

    println!(
        "running: {}\ntype: {:?}\npath: {:?}\nparams: {params:?}\n",
        runnable.name, runnable.rtype, runnable.path
    );

    let command = match params {
        RunnableParams::Rust { release, args, test } => run_rust_command(runnable, *release, *test, args),
        RunnableParams::Javascript {} => {
            todo!()
        }
    };

    run_command_pipe_to_terminal(&command);

    println!("\nPress ENTER to close");
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();

    Ok(())
}
