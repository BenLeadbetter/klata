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