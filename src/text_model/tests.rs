use super::*;

#[test]
fn new_character_is_untyped() {
    let character = Character::new('b');
    assert_eq!(character.status(), CharacterStatus::Untyped);
}

#[test]
fn correctly_attempted_character_is_correct() {
    let mut character = Character::new('b');
    character.attempt('b');
    assert_eq!(character.status(), CharacterStatus::Correct);
}

#[test]
fn incorrectly_attempted_character_is_wrong() {
    let mut character = Character::new('b');
    character.attempt('x');
    assert_eq!(character.status(), CharacterStatus::Wrong);
}

#[test]
fn erased_character_is_untyped() {
    let mut character = Character::new('b');
    character.attempt('x');
    character.erase();
    assert_eq!(character.status(), CharacterStatus::Untyped);
}

#[test]
fn correctly_reattempted_character_is_corrected() {
    let mut character = Character::new('b');
    character.attempt('x');
    character.erase();
    character.attempt('b');
    assert_eq!(character.status(), CharacterStatus::Corrected);
}

#[test]
fn incorrectly_reattempted_character_is_wrong() {
    let mut character = Character::new('b');
    character.attempt('x');
    character.erase();
    character.attempt('z');
    assert_eq!(character.status(), CharacterStatus::Wrong);
}

#[test]
fn plain_alphanumeric_is_valid() {
    let file_str = "<klata_text><text>Hello World</text></klata_text>";
    let _ = data::Data::from_string(file_str).unwrap();
}

#[test]
fn text_with_punctuation_is_valid() {
    let file_str = "<klata_text><text>#e(lo, W*rld!?</text></klata_text>";
    let _ = data::Data::from_string(file_str).unwrap();
}

#[test]
fn text_with_line_breaks_is_valid() {
    let file_str = "<klata_text><text>Hello\nWorld</text></klata_text>";
    let _ = data::Data::from_string(file_str).unwrap();
}

#[test]
fn text_with_directed_speech_marks_not_valid() {
    let file_str = "<klata_text><text>\u{201c}</text></klata_text>";
    let data = data::Data::from_string(file_str);
    assert_eq!(&format!("{}",data.unwrap_err()), "Unsupported character '\u{201c}'")
}

#[test]
fn text_with_directed_apostrophes_not_valid() {
    let file_str = "<klata_text><text>Ben\u{2019}s</text></klata_text>";
    let data = data::Data::from_string(file_str);
    assert_eq!(&format!("{}",data.unwrap_err()), "Unsupported character '\u{2019}'")
}

#[test]
fn leading_whitespace_trimmed() {
    let file_str = "<klata_text><text> Dredd</text></klata_text>";
    let data = data::Data::from_string(file_str).unwrap();
    assert_eq!(data.text, "Dredd".to_string());
}

#[derive(Default)]
struct TestObserver {
    events: Vec<Event>,
}

impl Observer for TestObserver {
    fn notify(&mut self, event: &Event) {
        self.events.push(event.clone());
    }
}

#[test]
fn observer_is_notified_on_typed_character() {
    use std::{
        cell::RefCell,
        rc::Rc,
    };
    let observer = Rc::<RefCell::<TestObserver>>::new(RefCell::<TestObserver>::new(Default::default()));
    let file_str = "<klata_text><text>wah-blow</text></klata_text>";
    let mut text = TextModel::from_string(file_str).unwrap();
    let weak = Rc::downgrade(&observer);
    text.register_observer(weak);

    text.type_character('w');
    
    assert_eq!(
        observer.borrow().events, 
        vec![Event::Type(('w', CharacterStatus::Correct))]
    );
}

#[test]
fn observer_is_notified_on_backspace() {
    use std::{
        cell::RefCell,
        rc::Rc,
    };
    let observer = Rc::<RefCell::<TestObserver>>::new(RefCell::<TestObserver>::new(Default::default()));
    let file_str = "<klata_text><text>wah-blow</text></klata_text>";
    let mut text = TextModel::from_string(file_str).unwrap();
    let weak = Rc::downgrade(&observer);
    text.register_observer(weak);

    text.type_character('x');
    text.backspace();
    
    assert_eq!(
        observer.borrow().events, 
        vec![
            Event::Type(('x', CharacterStatus::Wrong)),
            Event::Backspace
        ]
    );
}
