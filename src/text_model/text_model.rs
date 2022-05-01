use serde::{ Serialize, Deserialize, };
use std::{
    boxed::Box,
    error::Error,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CharacterStatus {
    Untyped,
    Correct,
    Corrected,
    Wrong,
}

pub struct Character {
    value: char,
    typed_value: Option<char>,
    wrong_attempts: u32,
}

impl Character {
    pub fn new(value: char) -> Character {
        Character {
            value,
            typed_value: None,
            wrong_attempts: 0,
        }
    }

    pub fn value(&self) -> char {
        self.value
    }

    pub fn status(&self) -> CharacterStatus {
        if let Some(typed) = self.typed_value {
            match self.wrong_attempts {
                0 => CharacterStatus::Correct,
                _ => {
                    if typed == self.value {
                        CharacterStatus::Corrected
                    } else {
                        CharacterStatus::Wrong
                    }
                }
            }
        } else {
            CharacterStatus::Untyped
        }
    }

    pub fn attempt(&mut self, c: char) {
        self.typed_value = Some(c);
        if c != self.value {
            self.wrong_attempts += 1;
        }
    }

    pub fn erase(&mut self) {
        self.typed_value = None;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Backspace,
    Type((char, CharacterStatus)),
}

pub trait Observer {
    fn notify(&mut self, event: &Event);
}

#[derive(Default)]
pub struct TextModel {
    buffer: Vec<Character>,
    cursor: usize,
    _author: Option<String>,
    _date: Option<[u16;3]>,
    observers: Vec<std::rc::Weak<std::cell::RefCell<dyn Observer>>>,
}

impl TextModel {
    pub fn from_string(s: &str) -> Result<TextModel, Box<dyn Error>> {
        let data = super::data::Data::from_string(s)?;
        let buffer = data.text.chars().map(|c| Character::new(c)).collect::<Vec<_>>();
        let cursor = 0;
        Ok(TextModel { 
            buffer, 
            cursor, 
            _author: data.author, 
            _date: data.date,
            observers: Vec::new(),
        })
    }

    pub fn characters(&self) -> std::slice::Iter<Character> {
        self.buffer.iter()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn type_character(&mut self, c: char) {
        if self.buffer.len() <= self.cursor {
            return;
        }
        self.buffer[self.cursor].attempt(c);
        self.notify_observers(
            Event::Type((c, self.buffer[self.cursor].status()))
        );
        self.cursor = self.cursor + 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.buffer[self.cursor].erase();
        self.notify_observers(Event::Backspace{});
    }

    pub fn register_observer(
        &mut self, observer: 
        std::rc::Weak<std::cell::RefCell<dyn Observer>>
    ) {
        self.observers.push(observer);
    }
    
    fn notify_observers(&mut self, event: Event) {
        self.observers = std::mem::take(&mut self.observers)
            .into_iter()
            .filter(|o| o.upgrade().is_some())
            .collect();
        for observer in self.observers.iter() {
            observer
                .upgrade()
                .unwrap()
                .borrow_mut()
                .notify(&event);
        }
    }
}
