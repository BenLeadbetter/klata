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
fn text_from_string_with_directed_speech_marks_neutralised() {
    let text = TextModel::from_string("\u{201c}Hello, World!\u{201d}".to_string());
    assert_eq!(text.buffer[0].value(), '\u{0022}');
    assert_eq!(text.buffer[14].value(), '\u{0022}');
}

#[test]
fn text_from_string_with_directed_apostrophes_neutralised() {
    let text = TextModel::from_string("Ben\u{2019}s".to_string());
    assert_eq!(text.buffer[3].value(), '\'');
}