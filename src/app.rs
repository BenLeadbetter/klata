use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use crate::widgets::Paragraph;

use tui::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    Frame,
    Terminal,
};

pub struct App {
    text: crate::text::Text,
}

pub type AppError = std::boxed::Box<dyn std::error::Error>;

impl App {
    pub fn from_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<App, AppError> {
        let file_content = std::str::from_utf8(&std::fs::read(path)?)?.to_string();
        Ok(App {
            text: crate::text::Text::from_string(file_content),
        })
    }

    pub fn run<W: std::io::Write>(mut self, buffer: W) -> Result<(), AppError> {
            let mut terminal = create_terminal(buffer)?;
        loop {
            terminal.draw(|f| self.ui(f))?;
            let event = event::read()?;
            match event  {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => { break; },
                    KeyCode::Char(c) => {
                        self.text.type_character(c);
                    },
                    KeyCode::Backspace => { 
                        self.text.backspace();
                    }
                    _ => {},

                },
                _ => {}
            }
        }
        teardown_terminal(terminal)?;
        Ok(())
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        f.render_widget(Paragraph::new(&self.text), f.size());
    }
}

fn create_terminal<W: std::io::Write>(mut buffer: W) -> Result<Terminal<CrosstermBackend<W>>, std::io::Error> {
    enable_raw_mode()?;
    execute!(buffer, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(buffer);
    Ok(Terminal::new(backend)?)
}

fn teardown_terminal<W: std::io::Write>(
    mut terminal: Terminal<CrosstermBackend<W>>
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
