use ratatui::{
  prelude::{Constraint, Direction, Layout, Margin, Rect},
  style::{Color, Style, Stylize},
  text::{Line, Span},
  widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
  Frame,
};

use crate::{
  helpers::runnable_path_display,
  sources::runfile::RunFileParams,
  state::{Mode, State},
  types::RunnableParams,
};

pub fn render(frame: &mut Frame, state: &mut State, root_path: &str) -> anyhow::Result<()> {
  let frame_size = frame.area().inner(Margin::new(1, 1));

  render_bounder(frame, root_path, frame_size);

  let v_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![Constraint::Max(3), Constraint::Min(0)])
    .margin(1)
    .split(frame_size);

  render_search(frame, state, v_layout[0]);

  let h_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![Constraint::Min(64), Constraint::Percentage(100)])
    .split(v_layout[1]);

  render_list(frame, state, &h_layout);
  render_info(frame, state, root_path, &h_layout)?;

  Ok(())
}

fn render_bounder(frame: &mut Frame, root_path: &str, frame_size: Rect) {
  let border = Block::default()
    .title(Span::styled(
      "runnables-cli",
      Style::default().light_blue().bold(),
    ))
    .title(Span::styled(root_path, Style::default().bold()).into_right_aligned_line())
    .title_bottom(
      Span::styled("press 'q' to quit", Style::default().bold()).into_right_aligned_line(),
    );

  frame.render_widget(border, frame_size);
}

fn render_search(frame: &mut Frame, state: &State, frame_size: Rect) {
  let value = state.search.value();
  let value = if state.mode == Mode::List && value.is_empty() {
    "press TAB to search"
  } else {
    value
  };
  let search = Paragraph::new(value)
    .style(match state.mode {
      Mode::Search => Style::default().fg(state.args.color),
      Mode::List => Style::default(),
    })
    .block(Block::default().title("search").borders(Borders::ALL));
  frame.render_widget(search, frame_size);
  if state.mode == Mode::Search {
    frame.set_cursor_position((
      // Put cursor past the end of the input text
      frame_size.x + state.search.visual_cursor() as u16 + 1,
      // Move one line down, from the border to the input line
      frame_size.y + 1,
    ));
  }
}

fn render_list(frame: &mut Frame, state: &mut State, layout: &[Rect]) {
  state.set_active_runnables();

  let table_items = state.active.iter().enumerate().map(|(i, runnable)| {
    let row = vec![
      runnable
        .display_name
        .clone()
        .unwrap_or(runnable.name.clone())
        .light_blue(),
      Span::from(runnable.params.to_string()),
      Span::from(runnable.aliases.join(", ")),
    ];
    if i == state.selected {
      Row::new(row).bold().underlined()
    } else {
      Row::new(row)
    }
  });

  let table = Table::new(
    table_items,
    [
      Constraint::Fill(2),
      Constraint::Fill(1),
      Constraint::Fill(1),
    ],
  )
  .header(Row::new(["Name", "Type", "Alias"]).dim().underlined())
  .block(Block::default().borders(Borders::ALL).fg(match state.mode {
    Mode::List => state.args.color,
    Mode::Search => Color::White,
  }));

  frame.render_widget(table, layout[0]);
}

fn render_info(
  frame: &mut Frame,
  state: &State,
  root_path: &str,
  layout: &[Rect],
) -> anyhow::Result<()> {
  let mut lines: Vec<Line> = Vec::new();

  match state.active.get(state.selected) {
    Some(selected) => {
      lines.push(Line::from(vec![
        Span::from("name: "),
        Span::from(&selected.name).light_blue().bold(),
      ]));
      if !selected.aliases.is_empty() {
        lines.push(Line::from(vec![
          if selected.aliases.len() > 1 {
            Span::from("aliases: ")
          } else {
            Span::from("alias: ")
          },
          Span::from(selected.aliases.join(", ")).light_blue().bold(),
        ]));
      }

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

      if let Some(after) = &selected.after {
        lines.push(Line::from(vec![
          Span::from("after: "),
          Span::from(format!("{after:?}")).light_blue().bold(),
        ]));
      }

      let description = selected
        .description
        .as_ref()
        .unwrap_or(&"-- NO DESCRIPTION --".to_string())
        .clone();
      lines.push(Line::from(""));
      lines.push(Line::from(description));

      if let RunnableParams::RunFile(RunFileParams { command }) = &selected.params {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::from(command).light_blue().bold()));
      }

      lines.push(Line::from(""));
      lines.extend(keypress_helper(&selected.params));
    }
    None => lines.push(Line::from("-- NO RUNNABLE SELECTED --")),
  }
  let info = Paragraph::new(lines)
    .block(Block::default().borders(Borders::ALL))
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
        Span::from("r").bold().light_blue(),
        Span::from(": run"),
      ]),
    ],
    RunnableParams::Shell(_) => vec![
      // Line::from("actions:"),
      // Line::from(""),
      Line::from(vec![
        Span::from("r").bold().light_blue(),
        Span::from(": run"),
      ]),
    ],
    // RunnableParams::Javascript(_) => vec![
    //   // Line::from("actions:"),
    //   // Line::from(""),
    //   Line::from(vec![
    //     Span::from("y").bold().light_blue(),
    //     Span::from(": yarn"),
    //   ]),
    //   Line::from(vec![
    //     Span::from("n").bold().light_blue(),
    //     Span::from(": npm"),
    //   ]),
    // ],
    RunnableParams::RustBin(_) => vec![
      Line::from(vec![
        Span::from("r").bold().light_blue(),
        Span::from(": run"),
      ]),
      Line::from(vec![
        Span::from("R").bold().light_blue(),
        Span::from(": run release"),
      ]),
      Line::from(vec![
        Span::from("p").bold().light_blue(),
        Span::from(": publish"),
      ]),
      Line::from(vec![
        Span::from("i").bold().light_blue(),
        Span::from(": install"),
      ]),
      Line::from(vec![
        Span::from("b").bold().light_blue(),
        Span::from(": build"),
      ]),
      Line::from(vec![
        Span::from("B").bold().light_blue(),
        Span::from(": build release"),
      ]),
      Line::from(vec![
        Span::from("t").bold().light_blue(),
        Span::from(": test"),
      ]),
      Line::from(vec![
        Span::from("c").bold().light_blue(),
        Span::from(": check"),
      ]),
      Line::from(vec![
        Span::from("C").bold().light_blue(),
        Span::from(": clippy"),
      ]),
      Line::from(vec![
        Span::from("f").bold().light_blue(),
        Span::from(": format"),
      ]),
    ],
    RunnableParams::RustLib(_) => vec![
      Line::from(vec![
        Span::from("p").bold().light_blue(),
        Span::from(": publish"),
      ]),
      Line::from(vec![
        Span::from("b").bold().light_blue(),
        Span::from(": build"),
      ]),
      Line::from(vec![
        Span::from("B").bold().light_blue(),
        Span::from(": build release"),
      ]),
      Line::from(vec![
        Span::from("t").bold().light_blue(),
        Span::from(": test"),
      ]),
      Line::from(vec![
        Span::from("c").bold().light_blue(),
        Span::from(": check"),
      ]),
      Line::from(vec![
        Span::from("C").bold().light_blue(),
        Span::from(": clippy"),
      ]),
      Line::from(vec![
        Span::from("f").bold().light_blue(),
        Span::from(": format"),
      ]),
    ],
    RunnableParams::None => {
      panic!("tried to get keypress helpers for None variant")
    }
  }
}
