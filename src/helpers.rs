use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Context;
use path_clean::PathClean;

pub fn absolute_path(path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .context("failed to get current directory from env")?
            .join(path)
    }
    .clean();
    Ok(absolute_path)
}

pub fn runnable_path_display(root_path: &str, path: impl AsRef<Path>) -> anyhow::Result<String> {
    let res = absolute_path(path)?
        .display()
        .to_string()
        .replace(root_path, "");
    Ok(res)
}

// pub fn make_cursive_app() -> CursiveRunnable {
//     let mut siv = cursive::default();
//     siv.add_global_callback('q', |s| s.quit());
//     siv.set_theme(Theme {
//         shadow: false,
//         borders: BorderStyle::Simple,
//         palette: Palette::retro().with(|palette| {
//             use cursive::theme::BaseColor::*;
//
//             {
//                 // First, override some colors from the base palette.
//                 use cursive::theme::Color::TerminalDefault;
//                 use cursive::theme::PaletteColor::*;
//
//                 palette[Background] = TerminalDefault;
//                 palette[View] = TerminalDefault;
//                 palette[Primary] = White.dark();
//                 palette[TitlePrimary] = Blue.light();
//                 palette[Secondary] = Blue.light();
//                 palette[Highlight] = Blue.dark();
//             }
//
//             {
//                 // Then override some styles.
//                 use cursive::theme::Effect::*;
//                 use cursive::theme::PaletteStyle::*;
//                 use cursive::theme::Style;
//                 palette[Highlight] = Style::from(Blue.light()).combine(Bold);
//             }
//         }),
//     });
//     siv
// }


