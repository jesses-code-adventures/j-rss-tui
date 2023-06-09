// Their stuff
use anyhow::Result;
use std::{io, time::Duration};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
// Our stuff
mod session_and_user;
mod feeds_and_entry;
mod ui;
mod app;
use app::App;
use crate::session_and_user::session::Session;

#[tokio::main]
async fn main() -> Result<()> {
    let tick_rate = Duration::from_millis(30);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let session = Session::load_from_json();
    let mut app = App::new(session.ok());
    terminal.clear()?;
    app.run(&mut terminal, tick_rate)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    terminal.set_cursor(0, 0)?;
    Ok(())
}
