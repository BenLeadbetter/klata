use super::{Finished, State};
use crate::text_model::TextModel;
use crate::widgets::TextArea;
use crossterm::event::{Event, KeyCode};
use std::{
    boxed::Box,
    error::Error,
};

#[derive(Default)]
pub struct Typing {
    text_model: Option<TextModel>,
}

impl Typing {
    pub fn new(text_file_content: &str) -> Result<Typing, Box<dyn Error>> {
        Ok(Typing {
            text_model: Some(TextModel::from_string(text_file_content)?),
        })
    }
    fn take(&mut self) -> Typing {
        Typing {
            text_model: self.text_model.take(),
        }
    }
}

impl<B> State<B> for Typing
where
    B: tui::backend::Backend,
{
    fn handle_event(&mut self, event: crossterm::event::Event) -> Box<dyn State<B>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => Box::new(Finished {}),
                KeyCode::Char(c) => {
                    self.text_model.as_mut().unwrap().type_character(c);
                    Box::new(self.take())
                }
                KeyCode::Backspace => {
                    self.text_model.as_mut().unwrap().backspace();
                    Box::new(self.take())
                }
                _ => Box::new(self.take()),
            },
            _ => Box::new(self.take()),
        }
    }
    fn ui(&self, frame: &mut tui::Frame<B>) {
        frame.render_widget(
            TextArea::new(self.text_model.as_ref().unwrap()),
            frame.size(),
        );
    }
}
