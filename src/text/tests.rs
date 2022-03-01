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