use std::{
    io::{self, Stdout},
    rc::Rc,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Terminal};

use crate::state::State;

mod ui;

pub fn run(state: Rc<State>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // RUN APP
    let res = render_loop(&mut terminal);

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

fn render_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
            frame.render_widget(greeting, frame.size());
        })?;
        if true {
            break;
        }
    }
    Ok(())
}
