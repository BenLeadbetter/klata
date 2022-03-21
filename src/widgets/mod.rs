use crate::text;

use tui::{
    layout::{ Alignment, Rect, },
    text::{ Span, Spans, },
    style::{ Color, Style, },
    widgets::{ self, Block, Borders, Wrap,},
};

pub struct Paragraph<'a> {
    characters: Spans<'a>,
}

impl<'a> Paragraph<'a> {
    pub fn new(text: &text::Text) -> Paragraph<'a> {
        Paragraph {
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

fn to_styled_char<'a>(c: char, status: text::CharacterStatus, is_cursor: bool) -> Span<'a> {
    Span{
        style: Style {
            fg: match status {
                text::CharacterStatus::Untyped => { Some(Color::DarkGray) },
                text::CharacterStatus::Correct => { Some(Color::White) },
                text::CharacterStatus::Corrected => { Some(Color::Green) },
                text::CharacterStatus::Wrong => { Some(Color::Red) },
            },
            bg: {
                if is_cursor {
                    Some(Color::White)
                } else if c == ' ' && status == text::CharacterStatus::Corrected {
                    Some(Color::Green)
                } else if c == ' ' && status == text::CharacterStatus::Wrong {
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

impl<'a> tui::widgets::Widget for Paragraph<'a> {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let paragraph_block = Block::default().borders(Borders::ALL);
        let paragraph = widgets::Paragraph::new(self.characters)
            .block(paragraph_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
