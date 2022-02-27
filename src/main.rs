use std::{
    io,
};
use tui::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    Frame,
    layout::{
        Layout, 
        Constraint, 
        Direction,
    },
    Terminal,
    widgets::{
       Block, 
       Borders,
    },
};
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
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

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let app = App{};
    let return_value = app.run(&mut terminal);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return_value
}

struct App {
}

impl App {
    fn run<B: Backend>(
        self, 
        terminal: &mut Terminal<B>
    ) -> Result<(), std::io::Error> {
        loop {
            terminal.draw(|f| ui(f, &self))?;
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
        Ok(())
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, _app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(30),
                Constraint::Percentage(5),
            ].as_ref()
        )
        .split(size);
    let paragraph_block = Block::default()
        .borders(Borders::ALL);
    let guide_block = Block::default()
        .borders(Borders::ALL);
    let status_bar = Block::default();
    f.render_widget(paragraph_block, chunks[0]);
    f.render_widget(guide_block, chunks[1]);
    f.render_widget(status_bar, chunks[2]);
}