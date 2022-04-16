use crate::app::states;

use crossterm::{
    event::{self, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

pub struct App<W>
where
    W: std::io::Write,
{
    state: std::boxed::Box<dyn states::State<CrosstermBackend<W>>>,
}
pub type AppError = std::boxed::Box<dyn std::error::Error>;

impl<W> App<W>
where
    W: std::io::Write,
{
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<App<W>, AppError> {
        let file_content = std::str::from_utf8(&std::fs::read(path)?)?.to_string();
        Ok(App::<W> {
            state: std::boxed::Box::new(states::Typing::new(&file_content)?),
        })
    }

    pub fn run(mut self, buffer: W) -> Result<(), AppError> {
        let mut terminal = create_terminal(buffer)?;
        loop {
            terminal.draw(|f| self.state.ui(f))?;
            self.state = self.state.handle_event(event::read()?);
            if self.state.terminate() {
                break;
            }
        }
        teardown_terminal(terminal)?;
        Ok(())
    }
}

fn create_terminal<W: std::io::Write>(
    mut buffer: W,
) -> Result<Terminal<CrosstermBackend<W>>, std::io::Error> {
    enable_raw_mode()?;
    execute!(buffer, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(buffer);
    Ok(Terminal::new(backend)?)
}

fn teardown_terminal<W: std::io::Write>(
    mut terminal: Terminal<CrosstermBackend<W>>,
) -> Result<(), std::io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
