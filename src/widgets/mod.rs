use crate::text::{
    CharacterStatus,
    Text,
};

use tui::{
    layout::{ Alignment, Rect, },
    text::{ Span, Spans, },
    style::{ Color, Style, },
    widgets::{ Block, Borders, Paragraph, Wrap,},
};

pub struct ParagraphWidget<'a> {
    text: &'a Text,
}

impl<'a> ParagraphWidget<'a> {
    pub fn new(text: &'a Text) -> ParagraphWidget<'a> {
        ParagraphWidget {
            text,
        }
    }
}

impl<'a> tui::widgets::Widget for ParagraphWidget<'a> {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let paragraph_block = Block::default().borders(Borders::ALL);
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
        paragraph.render(area, buf);
    }
}
