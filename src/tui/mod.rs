use std::{
  io::{self, Stdout},
  time::Duration,
};

use anyhow::Context;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::state::State;

mod ui;

pub fn run(state: &mut State) -> anyhow::Result<()> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let res = render_loop(&mut terminal, state);

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  if let Err(e) = res {
    println!("{e:#?}");
  }

  Ok(())
}

fn render_loop(
  terminal: &mut Terminal<CrosstermBackend<Stdout>>,
  state: &mut State,
) -> anyhow::Result<()> {
  let root_path = state.root_absolute_path()?;
  loop {
    terminal.draw(|frame| ui::render(frame, state, &root_path).expect("failed to draw frame"))?;
    if let Some(event) = poll_event()? {
      let should_break = state.handle_event(event);
      if should_break {
        break;
      }
    }
  }
  Ok(())
}

fn poll_event() -> anyhow::Result<Option<Event>> {
  if event::poll(Duration::from_millis(250)).context("event poll failed")? {
    let event = event::read().context("event read failed")?;
    Ok(Some(event))
  } else {
    Ok(None)
  }
}
