use super::*;
use tui::widgets::Widget;

#[test]
fn paragraph_from_empty_text_renders_empty_framed_block() {
    let text = text::Text::from_string("".to_string());
    let paragraph = Paragraph::new(&text);
    
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
    let text = text::Text::from_string("I am".to_string());
    let paragraph = Paragraph::new(&text);
    
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
    let text = text::Text::from_string("I am".to_string());
    let paragraph = Paragraph::new(&text);
    
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
    let mut text = text::Text::from_string("I am".to_string());
    text.type_character('x');

    let paragraph = Paragraph::new(&text);
    
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
    let mut text = text::Text::from_string("I am".to_string());
    text.type_character('x');
    text.backspace();
    text.type_character('I');

    let paragraph = Paragraph::new(&text);
    
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
fn cursor_rendered_white_bg_grey_fg() {
    let text = text::Text::from_string("I am".to_string());
    let paragraph = Paragraph::new(&text);
    
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
    let mut text = text::Text::from_string("I am".to_string());
    text.type_character('I');
    text.type_character(' ');
    text.type_character('a');
    text.type_character('m');
    let paragraph = Paragraph::new(&text);
    
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
