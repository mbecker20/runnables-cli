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

use crate::state::State;

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &State, root_path: &str) {
    let frame_size = frame.size().inner(&Margin::new(1, 1));

    let border = Block::default()
        .title(Span::styled(
            "runnables-cli",
            Style::default().blue().bold(),
        ))
        .title(
            Title::from(Span::styled(root_path, Style::default().bold()))
                .alignment(Alignment::Right),
        ).title(
            Title::from(Span::styled("press 'q' to quit", Style::default().bold()))
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        );

    frame.render_widget(border, frame_size.clone());

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame_size.inner(&Margin::new(1, 1)));

    let mut lines: Vec<Line> = Default::default();

    for (index, runnable) in state.runnables.iter().enumerate() {
        let mut line = Line::from(vec![
            Span::styled(runnable.params.to_string(), Style::default().dim()),
            Span::from(" => ").dim(),
            runnable.name.blue(),
            Span::from(" => ").dim(),
            Span::from(runnable.path.to_str().unwrap()),
        ]);
        if state.selected == index {
            line.patch_style(Style::default().bold().underlined())
        }
        lines.push(line);
    }

    let list =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL));

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
