use std::io::{stdout, Error};
use crate::csv_reader;
use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, widgets::ListState, Terminal
};

use super::company_list::company_list;

pub fn main_view() -> Result<(), Error> {
    let companies = csv_reader()?;
    //put into app state
    let mut state = ListState::default();
    
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_stateful_widget(
        company_list(&companies),
                area,
                &mut state
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                else if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                    state.select_next();
                }

                else if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
                    state.select_previous();
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}