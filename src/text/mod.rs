#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
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
    fn new(value: char) -> Character {
        Character {
            value,
            typed_value: None,
            wrong_attempts: 0,
        }
    }
    
    pub fn value(&self) -> char { self.value }
    
    pub fn status(&self) -> CharacterStatus { 
        if let Some(typed) = self.typed_value {
            match self.wrong_attempts {
                0 => { CharacterStatus::Correct }
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
    
    fn attempt(&mut self, c: char) {
        self.typed_value = Some(c);
        if c != self.value {
            self.wrong_attempts += 1;
        }
    }
    
    fn erase(&mut self) {
        self.typed_value = None;
    }
}

pub struct Text {
    buffer: Vec<Character>,
    cursor: usize,
}

impl Text {
    pub fn from_string(s: String) -> Text {
        let buffer = s
            .chars()
            .map(|c| Character::new(c)) 
            .collect::<Vec<_>>();
        let cursor = 0;
        Text {
            buffer,
            cursor,
        }
    }

    pub fn characters(&self) -> std::slice::Iter<Character> {
        self.buffer.iter()
    }

    pub fn cursor(&self) -> usize { self.cursor }
    
    pub fn type_character(&mut self, c: char) {
        if self.buffer.len() <= self.cursor {
            return;
        }
        self.buffer[self.cursor].attempt(c);
        self.cursor = self.cursor + 1;
    }
    
    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.buffer[self.cursor].erase();
    }
}