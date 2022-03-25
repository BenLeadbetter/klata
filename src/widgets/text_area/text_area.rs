use crate::text_model;
use super::styled_char::StyledChar;
use tui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, },
};

pub struct TextArea {
    characters: Vec<StyledChar>,
}

impl TextArea {
    pub fn new(text: &text_model::TextModel) -> TextArea {
        TextArea {
            characters: text.characters()
                .map(|c| StyledChar::from(c))
                .enumerate()
                .map(|(i, c)| c.cursor(i == text.cursor()))
                .collect::<Vec<_>>(),
        }
    }
}

impl<'a> tui::widgets::Widget for TextArea {
    fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
        let borders = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner = borders.inner(area);
        borders.render(area, buf);
        
        let lines = super::reflow::reflow(
            &self.characters,
            inner.width.into(),
            |sc| sc.c == ' ',
            |sc| sc.c == '\n',
        );
        
        let mut lines_iter = lines.iter();
        for j in 0..inner.height {
            let line = lines_iter.next();
            for i in 0..inner.width {
                match line {
                    Some(l) => {
                        if (i as usize) < l.len() {
                            let styled_char: &StyledChar = &l[i as usize];
                            buf.set_string(
                                inner.x + i, 
                                inner.y + j, 
                                styled_char.c.to_string(), 
                                styled_char.style)
                        }
                    },
                    None => {},
                } 
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui::widgets::Widget;

    fn empty_text_model() -> text_model::TextModel {
        let empty_file_str = "<klata_text><text></text></klata_text>";
        text_model::TextModel::from_string(empty_file_str).unwrap()
    }

    fn make_text_model() -> text_model::TextModel {
        let file_str = "<klata_text><text>I am</text></klata_text>";
        text_model::TextModel::from_string(file_str).unwrap()
    }

    #[test]
    fn paragraph_from_empty_text_renders_empty_framed_block() {
        let text = empty_text_model();
        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        let mut expected = tui::buffer::Buffer::empty(rect.clone());
        let framed_block = tui::widgets::Block::default()
            .borders(tui::widgets::Borders::ALL)
            .border_type(tui::widgets::BorderType::Rounded);
        framed_block.render(rect.clone(), &mut expected);

        assert_eq!(expected, buffer);
    }

    #[test]
    fn text_wraps_and_left_aligns() {
        let text = make_text_model();
        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].symbol, "I".to_string());
        assert_eq!(buffer.content[6].symbol, " ".to_string());
        assert_eq!(buffer.content[9].symbol, "a".to_string());
        assert_eq!(buffer.content[10].symbol, "m".to_string());
    }

    #[test]
    fn untyped_text_rendered_grey() {
        let text = make_text_model();
        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].fg, tui::style::Color::DarkGray);
        assert_eq!(buffer.content[9].fg, tui::style::Color::DarkGray);
        assert_eq!(buffer.content[10].fg, tui::style::Color::DarkGray);
    }

    #[test]
    fn incorrect_text_rendered_red() {
        let mut text = make_text_model();
        text.type_character('x');

        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].fg, tui::style::Color::Red);
    }

    #[test]
    fn corrected_text_rendered_green() {
        let mut text = make_text_model();
        text.type_character('x');
        text.backspace();
        text.type_character('I');

        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].fg, tui::style::Color::Green);
    }

    #[test]
    fn incorrect_space_rendered_bg_red() {
        let mut text = make_text_model();
        text.type_character('I');
        text.type_character('x');

        let paragraph = TextArea::new(&text);
        
        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);
        
        assert_eq!(buffer.content[6].bg, tui::style::Color::Red);
    }

    #[test]
    fn corrected_space_rendered_bg_green() {
        let mut text = make_text_model();
        text.type_character('I');
        text.type_character('x');
        text.backspace();
        text.type_character(' ');

        let paragraph = TextArea::new(&text);
        
        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);
        
        assert_eq!(buffer.content[6].bg, tui::style::Color::Green);
    }

    #[test]
    fn cursor_rendered_white_bg_grey_fg() {
        let text = make_text_model();
        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].fg, tui::style::Color::DarkGray);
        assert_eq!(buffer.content[5].bg, tui::style::Color::White);
    }

    #[test]
    fn correctly_type_text_text_is_rendered_white() {
        let mut text = make_text_model();
        text.type_character('I');
        text.type_character(' ');
        text.type_character('a');
        text.type_character('m');
        let paragraph = TextArea::new(&text);

        let rect = tui::layout::Rect {
            width: 4,
            height: 4,
            ..Default::default()
        };
        let mut buffer = tui::buffer::Buffer::empty(rect.clone());
        paragraph.render(rect.clone(), &mut buffer);

        assert_eq!(buffer.content[5].fg, tui::style::Color::White);
        assert_eq!(buffer.content[9].fg, tui::style::Color::White);
        assert_eq!(buffer.content[10].fg, tui::style::Color::White);
    }
}