#[cfg(test)]
mod tests;

use crate::text_model;

use tui::{
    layout::{ Alignment, Rect, },
    text::{ Span, Spans, },
    style::{ Color, Style, },
    widgets::{ self, Block, Borders, BorderType, Wrap,},
};

pub struct TextArea<'a> {
    characters: Spans<'a>,
}

impl<'a> TextArea<'a> {
    pub fn new(text: &text_model::TextModel) -> TextArea<'a> {
        TextArea {
            characters: Spans::<'a>::from(
                text
                    .characters()
                    .enumerate()
                    .map(|(i, c)| to_styled_char(c.value(), c.status(), i == text.cursor()))
                    .collect::<Vec<_>>(),
            )
        }
    }
}

fn to_styled_char<'a>(c: char, status: text_model::CharacterStatus, is_cursor: bool) -> Span<'a> {
    Span {
        style: Style {
            fg: match status {
                text_model::CharacterStatus::Untyped => { Some(Color::DarkGray) },
                text_model::CharacterStatus::Correct => { Some(Color::White) },
                text_model::CharacterStatus::Corrected => { Some(Color::Green) },
                text_model::CharacterStatus::Wrong => { Some(Color::Red) },
            },
            bg: {
                if is_cursor {
                    Some(Color::White)
                } else if c == ' ' && status == text_model::CharacterStatus::Corrected {
                    Some(Color::Green)
                } else if c == ' ' && status == text_model::CharacterStatus::Wrong {
                    Some(Color::Red)
                } else {
                    None
                }
            },
            ..Style::default()
        },
        content: std::borrow::Cow::from(c.to_string()),
    }
}

impl<'a> tui::widgets::Widget for TextArea<'a> {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let paragraph_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let paragraph = widgets::Paragraph::new(self.characters)
            .block(paragraph_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
