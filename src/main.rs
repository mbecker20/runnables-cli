use std::{io::Read, path::PathBuf, str::FromStr};

use anyhow::Context;
use clap::Parser;
use helpers::absolute_path;
use sources::get_runnables;

use crate::sources::run_runnable;

mod components;
mod helpers;
mod runnables;
mod sources;
mod state;
mod tui;
mod types;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct RunnablesCli {
    #[arg(default_value_t = String::from("."))]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let args = RunnablesCli::parse();
    let path = PathBuf::from_str(&args.path)?;
    let root_path_display = absolute_path(&path)?.display().to_string();

    let state = state::State::rc();

    let runnables = get_runnables(&path);

    // run tui app which attached runnable to state then finishes
    match tui::run(state.clone()).context("failure in tui app") {
        Ok(_) => {
            let runnable = state.get_runnable();
            runnable.log_info();
            run_runnable(runnable);
        }
        Err(e) => {
            println!("\n{e:#?}");
        }
    }

    println!("\nPress ENTER to close");
    let buffer = &mut [0u8];
    std::io::stdin()
        .read_exact(buffer)
        .context("failed to read ENTER")?;

    Ok(())
}

// fn main() -> anyhow::Result<()> {
//     let args = RunnablesCli::parse();
//     let path = PathBuf::from_str(&args.path)?;
//     let root_path = absolute_path(&args.path)?.display().to_string();
//
//     let rust_runnables = Rust::find_runnables(&path);
//     let root_path = format!("{root_path}/");
//
//     let mut siv = make_cursive_app();
//
//     let state: Rc<RefCell<Option<State>>> = Default::default();
//
//     let mut runnables = LinearLayout::vertical();
//
//     for runnable in rust_runnables {
//         let text = format!(
//             " {:?} -> {} -> {} ",
//             runnable.rtype,
//             runnable.name,
//             runnable_path_display(&root_path, &runnable.path)?
//         );
//         let state = state.clone();
//         runnables.add_child(Button::new(text.clone(), move |s| {
//             let mut dialog = Dialog::text(text.clone())
//                 .title("choose params")
//                 .padding_lrtb(2, 2, 2, 2);
//
//             let _state = state.clone();
//             let _runnable = runnable.clone();
//             dialog.add_button("debug", move |s| {
//                 s.quit();
//                 _state.replace(Some(State {
//                     runnable: _runnable.clone(),
//                     params: RunnableParams::Rust {
//                         release: false,
//                         test: false,
//                         args: None,
//                     },
//                 }));
//             });
//
//             let _state = state.clone();
//             let _runnable = runnable.clone();
//             dialog.add_button("release", move |s| {
//                 s.quit();
//                 _state.replace(Some(State {
//                     runnable: _runnable.clone(),
//                     params: RunnableParams::Rust {
//                         release: true,
//                         test: false,
//                         args: None,
//                     },
//                 }));
//             });
//
//             let _state = state.clone();
//             let _runnable = runnable.clone();
//             dialog.add_button("test", move |s| {
//                 s.quit();
//                 _state.replace(Some(State {
//                     runnable: _runnable.clone(),
//                     params: RunnableParams::Rust {
//                         release: false,
//                         test: true,
//                         args: None,
//                     },
//                 }));
//             });
//
//             s.add_layer(dialog);
//         }));
//     }
//
//     let dialog = Dialog::around(runnables)
//         .title(format!(
//             "runnables -> ( {} )",
//             absolute_path(&args.path)?.display()
//         ))
//         .title_position(HAlign::Center)
//         .padding_lrtb(2, 2, 2, 2);
//
//     siv.add_layer(dialog);
//
//     siv.run();
//
//     let state = state.borrow();
//
//     if state.is_none() {
//         return Ok(());
//     }
//
//     let State { runnable, params } = state.as_ref().unwrap();
//
//     println!(
//         "running: {}\ntype: {:?}\npath: {:?}\nparams: {params:?}\n",
//         runnable.name, runnable.rtype, runnable.path
//     );
//
//     let command = match params {
//         RunnableParams::Rust {
//             release,
//             args,
//             test,
//         } => run_rust_command(runnable, *release, *test, args),
//         RunnableParams::Javascript {} => {
//             todo!()
//         }
//     };
//
//     run_command_pipe_to_terminal(&command);
//
//     println!("\nPress ENTER to close");
//     let buffer = &mut [0u8];
//     std::io::stdin().read_exact(buffer).unwrap();
//
//     Ok(())
// }
