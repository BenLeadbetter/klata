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
    style::{
        Color,
        Style,
    },
    widgets::{
       Block, 
       Borders,
       Paragraph,
       Wrap,
    },
    Terminal,
};

use crate::text::{
    CharacterStatus,
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
        let size = f.size();
        let paragraph_block = Block::default()
            .borders(Borders::ALL);
        let to_styled_char = |(i, c): (usize, &crate::text::Character)| -> Span {
            let style = Style {
                fg: match c.status() {
                    crate::text::CharacterStatus::Untyped => { Some(Color::DarkGray) },
                    crate::text::CharacterStatus::Correct => { Some(Color::White) },
                    crate::text::CharacterStatus::Corrected => { Some(Color::Green) },
                    crate::text::CharacterStatus::Wrong => { Some(Color::Red) },
                },
                bg: {
                    if i == self.text.cursor() {
                        Some(Color::White)
                    } else if c.value() == ' ' && c.status() == CharacterStatus::Corrected {
                        Some(Color::Green)
                    } else if c.value() == ' ' && c.status() == CharacterStatus::Wrong {
                        Some(Color::Red)
                    } else {
                        None
                    }
                },
                ..Style::default()
            };
            Span::styled(c.value().to_string(), style)
        };
        let styled_characters = self.text
            .characters()
            .enumerate()
            .map(to_styled_char)
            .collect::<Vec<_>>();
        let paragraph = Paragraph::new(Spans::from(styled_characters))
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
