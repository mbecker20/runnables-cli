use ratatui::{
    prelude::{Alignment, Backend, Constraint, Direction, Layout, Margin},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph,
    },
    Frame,
};

use crate::{
    state::State,
    types::{Runnable, RunnableParamsVariant},
};

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &State, root_path: &str) {
    let frame_size = frame.size().inner(&Margin::new(1, 1));

    let selected_variant: RunnableParamsVariant = (&state.runnables[state.selected].params).into();

    let border = Block::default()
        .title(Span::styled(
            "runnables-cli",
            Style::default().blue().bold(),
        ))
        .title(
            Title::from(Span::styled(root_path, Style::default().bold()))
                .alignment(Alignment::Right),
        )
        .title(Title::from(keypress_helper(selected_variant)).position(Position::Bottom))
        .title(
            Title::from(Span::styled("press 'q' to quit", Style::default().bold()))
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        );

    frame.render_widget(border, frame_size);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame_size.inner(&Margin::new(1, 1)));

    let mut lines: Vec<Line> = Default::default();

    let runfile_runnables = state.get_runnables_variants(RunnableParamsVariant::RunFile);
    if !runfile_runnables.is_empty() {
        lines.push(Line::from("-------- runfile ---------"));
        for runnable in runfile_runnables {
            let line = runnable_line(runnable, runnable.index == state.selected);
            lines.push(line);
        }
    }

    let rust_runnables = state.get_runnables_variants(RunnableParamsVariant::Rust);
    if !rust_runnables.is_empty() {
        lines.push(Line::from("---------- rust ---------"));
        for runnable in rust_runnables {
            let line = runnable_line(runnable, runnable.index == state.selected);
            lines.push(line);
        }
    }

    // for (index, runnable) in state.runnables.iter().enumerate() {
    //     let line = runnable_line(runnable, index == state.selected);
    //     lines.push(line);
    // }

    let list = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, layout[0]);

    let description = Paragraph::new(
        state.runnables[state.selected]
            .description
            .as_ref()
            .unwrap_or(&"-- NO DESCRIPTION --".to_string())
            .clone(),
    )
    .block(Block::default().title("description").borders(Borders::ALL));

    frame.render_widget(description, layout[1]);
}

fn runnable_line(runnable: &Runnable, selected: bool) -> Line {
    let mut line = Line::from(vec![
        // Span::styled(runnable.params.to_string(), Style::default().dim()),
        // Span::from(" => ").dim(),
        runnable.name.light_blue(),
        Span::from(" => ").gray(),
        Span::from(runnable.path.to_str().unwrap()).gray(),
    ]);
    if selected {
        line.patch_style(Style::default().bold().underlined());
    }

    line
}

fn keypress_helper(variant: RunnableParamsVariant) -> Line<'static> {
    match variant {
        RunnableParamsVariant::RunFile => Line::from(vec![
            Span::styled("enter", Style::default().bold().blue()),
            Span::from(": run"),
        ]),
        RunnableParamsVariant::Rust => Line::from(vec![
            Span::styled("enter", Style::default().bold().blue()),
            Span::from(": run, "),
            Span::styled("r", Style::default().bold().blue()),
            Span::from(": run release, "),
            Span::styled("b", Style::default().bold().blue()),
            Span::from(": build, "),
            Span::styled("B", Style::default().bold().blue()),
            Span::from(": build release, "),
            Span::styled("t", Style::default().bold().blue()),
            Span::from(": test, "),
            Span::styled("c", Style::default().bold().blue()),
            Span::from(": check, "),
            Span::styled("C", Style::default().bold().blue()),
            Span::from(": clippy, "),
            Span::styled("f", Style::default().bold().blue()),
            Span::from(": format, "),
        ]),
        // RunnableParamsVariant::Javascript => todo!(),
        RunnableParamsVariant::None => {
            panic!("tried to get keypress helpers for None variant")
        }
    }
}
