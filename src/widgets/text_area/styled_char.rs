use crate::text_model::{ Character, CharacterStatus, };
use tui::style::{ Color, Style, };

#[derive(Clone, PartialEq)]
pub struct StyledChar {
    pub style: Style,
    pub c: char,
}

impl StyledChar {
    pub fn cursor(self, is_cursor: bool) -> StyledChar {
        if !is_cursor {
            return self;
        }
        StyledChar {
            style: Style {
                bg: Some(Color::White),
                ..self.style
            },
            ..self
        }
    }
}

impl From<&Character> for StyledChar {
    fn from(c: &Character) -> StyledChar {
        StyledChar {
            style: Style {
                fg: match c.status() {
                    CharacterStatus::Untyped => Some(Color::DarkGray),
                    CharacterStatus::Correct => Some(Color::White),
                    CharacterStatus::Corrected => Some(Color::Green),
                    CharacterStatus::Wrong => Some(Color::Red),
                },
                bg: {
                    if c.value() == ' ' && c.status() == CharacterStatus::Corrected {
                        Some(Color::Green)
                    } else if c.value() == ' ' && c.status() == CharacterStatus::Wrong {
                        Some(Color::Red)
                    } else {
                        None
                    }
                },
                ..Style::default()
            },
            c: c.value(),
        }
    }
}