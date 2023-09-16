use ratatui::{
    prelude::{Alignment, Backend, Constraint, Direction, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::{
    helpers::runnable_path_display,
    state::State,
    types::{Runnable, RunnableParams, RunnableParamsVariant},
};

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    state: &State,
    root_path: &str,
) -> anyhow::Result<()> {
    let frame_size = frame.size().inner(&Margin::new(1, 1));

    render_bounder(frame, root_path, frame_size);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame_size.inner(&Margin::new(1, 1)));

    render_list(frame, state, &layout);
    render_info(frame, state, root_path, &layout)?;

    Ok(())
}

fn render_bounder<B: Backend>(frame: &mut Frame<B>, root_path: &str, frame_size: Rect) {
    let border = Block::default()
        .title(Span::styled(
            "runnables-cli",
            Style::default().light_blue().bold(),
        ))
        .title(
            Title::from(Span::styled(root_path, Style::default().bold()))
                .alignment(Alignment::Right),
        )
        .title(
            Title::from(Span::styled("press 'q' to quit", Style::default().bold()))
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        );

    frame.render_widget(border, frame_size);
}

fn render_list<B: Backend>(frame: &mut Frame<B>, state: &State, layout: &[Rect]) {
    let mut lines: Vec<Line> = Default::default();

    let runfile_runnables = state.get_runnables_variants(RunnableParamsVariant::RunFile);
    if !runfile_runnables.is_empty() {
        lines.push(Line::from("-------- runfile ---------"));
        // lines.push(Line::from(""));
        for runnable in runfile_runnables {
            let line = runnable_line(runnable, runnable.index == state.selected);
            lines.push(line);
        }
        lines.push(Line::from(""));
    }

    let rust_runnables = state.get_runnables_variants(RunnableParamsVariant::Rust);
    if !rust_runnables.is_empty() {
        lines.push(Line::from("---------- rust ---------"));
        // lines.push(Line::from(""));
        for runnable in rust_runnables {
            let line = runnable_line(runnable, runnable.index == state.selected);
            lines.push(line);
        }
        lines.push(Line::from(""));
    }

    let javascript_runnables = state.get_runnables_variants(RunnableParamsVariant::Javascript);
    if !javascript_runnables.is_empty() {
        lines.push(Line::from("---------- javascript ---------"));
        // lines.push(Line::from(""));
        for runnable in javascript_runnables {
            let line = runnable_line(runnable, runnable.index == state.selected);
            lines.push(line);
        }
        lines.push(Line::from(""));
    }

    // for (index, runnable) in state.runnables.iter().enumerate() {
    //     let line = runnable_line(runnable, index == state.selected);
    //     lines.push(line);
    // }

    let list = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, layout[0]);
}

fn runnable_line(runnable: &Runnable, selected: bool) -> Line {
    let mut line = Line::from(vec![
        // Span::styled(runnable.params.to_string(), Style::default().dim()),
        // Span::from(" => ").dim(),
        runnable.name.light_blue(),
        // Span::from(" => ").gray(),
        // Span::from(runnable.path.to_str().unwrap()).gray(),
    ]);
    if selected {
        line.patch_style(Style::default().bold().underlined());
    }

    line
}

fn render_info<B: Backend>(
    frame: &mut Frame<B>,
    state: &State,
    root_path: &str,
    layout: &[Rect],
) -> anyhow::Result<()> {
    let mut lines: Vec<Line> = Vec::new();

    let selected = &state.runnables[state.selected];

    lines.push(Line::from(vec![
        Span::from("name: "),
        Span::from(&selected.name).light_blue().bold(),
    ]));

    let path = runnable_path_display(root_path, &selected.path)?;
    lines.push(Line::from(vec![
        Span::from("path: "),
        Span::from(path).light_blue().bold(),
    ]));

    lines.push(Line::from(vec![
        Span::from("type: "),
        Span::from(format!("{}", selected.params))
            .light_blue()
            .bold(),
    ]));

    let description = selected
        .description
        .as_ref()
        .unwrap_or(&"-- NO DESCRIPTION --".to_string())
        .clone();
    lines.push(Line::from(""));
    lines.push(Line::from(description));

    lines.push(Line::from(""));
    lines.extend(keypress_helper(&selected.params));

    let info = Paragraph::new(lines)
        .block(Block::default().title("info").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(info, layout[1]);

    Ok(())
}

fn keypress_helper(params: &RunnableParams) -> Vec<Line<'static>> {
    match params {
        RunnableParams::RunFile(_) => vec![
            // Line::from("actions:"),
            // Line::from(""),
            Line::from(vec![
                Span::styled("r", Style::default().bold().light_blue()),
                Span::from(": run"),
            ]),
        ],
        RunnableParams::Javascript(_) => vec![
            // Line::from("actions:"),
            // Line::from(""),
            Line::from(vec![
                Span::styled("y", Style::default().bold().light_blue()),
                Span::from(": yarn"),
            ]),
            Line::from(vec![
                Span::styled("n", Style::default().bold().light_blue()),
                Span::from(": npm"),
            ]),
        ],
        RunnableParams::Rust(params) => {
            let mut first = if params.is_lib {
                vec![Line::from(vec![
                    Span::styled("p", Style::default().bold().light_blue()),
                    Span::from(": publish"),
                ])]
            } else {
                vec![
                    Line::from(vec![
                        Span::styled("r", Style::default().bold().light_blue()),
                        Span::from(": run"),
                    ]),
                    Line::from(vec![
                        Span::styled("R", Style::default().bold().light_blue()),
                        Span::from(": run release"),
                    ]),
                ]
            };
            let rest = vec![
                Line::from(vec![
                    Span::styled("b", Style::default().bold().light_blue()),
                    Span::from(": build"),
                ]),
                Line::from(vec![
                    Span::styled("B", Style::default().bold().light_blue()),
                    Span::from(": build release"),
                ]),
                Line::from(vec![
                    Span::styled("t", Style::default().bold().light_blue()),
                    Span::from(": test"),
                ]),
                Line::from(vec![
                    Span::styled("c", Style::default().bold().light_blue()),
                    Span::from(": check"),
                ]),
                Line::from(vec![
                    Span::styled("C", Style::default().bold().light_blue()),
                    Span::from(": clippy"),
                ]),
                Line::from(vec![
                    Span::styled("f", Style::default().bold().light_blue()),
                    Span::from(": format"),
                ]),
            ];
            first.extend(rest);
            first
        }
        RunnableParams::None => {
            panic!("tried to get keypress helpers for None variant")
        }
    }
}
