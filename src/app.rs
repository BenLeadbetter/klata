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

use tui::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    Frame,
    layout::Alignment,
    text::{
        Span,
        Spans,
    },
    widgets::{
       Block, 
       Borders,
       Paragraph,
       Wrap,
    },
    Terminal,
};

pub struct App {
    text: String,
    input_text: String,
}

pub type AppError = std::boxed::Box<dyn std::error::Error>;

impl App {
    pub fn from_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<App, AppError> {
        Ok(App {
            text: std::str::from_utf8(&std::fs::read(path)?)?
                .to_string(),
            input_text: String::new(),
        })
    }

    pub fn run<W: std::io::Write>(self, buffer: W) -> Result<(), AppError> {
            let mut terminal = create_terminal(buffer)?;
        loop {
            terminal.draw(|f| self.ui(f))?;
            let event = event::read()?;
            match event  {
                Event::Key(key) => {
                    if let KeyCode::Esc = key.code {
                        break;
                    }
                },
                _ => {}
            }
        }
        teardown_terminal(terminal)?;
        Ok(())
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();
        let paragraph_block = Block::default()
            .borders(Borders::ALL);
        let paragraph = Paragraph::new(Spans::from(Span::raw(self.text.clone())))
            .block(paragraph_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, size);
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
