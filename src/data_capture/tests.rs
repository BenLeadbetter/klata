use super::*;
use crate::text_model::TextModel;

#[test]
fn typing_on_model_gathers_data() {
    use std::{
        rc::Rc, cell::RefCell,
    };
    let data_capture = Rc::new(RefCell::new(DataCapture::<clock::TestClock>::new()));
    let file_str = "<klata_text><text>wah-blow</text></klata_text>";
    let mut model = TextModel::from_string(file_str).unwrap();
    let weak = Rc::downgrade(&data_capture);
    model.register_observer(weak);
    
    model.type_character('w');
    model.type_character('a');
    model.type_character('g');
    
    assert_eq!(
        data_capture.borrow().data,
        vec![
            (
                std::time::Duration::from_millis(0), 
                Data::CharacterTyped('w', text_model::CharacterStatus::Correct),
            ), (        
                std::time::Duration::from_millis(10), 
                Data::CharacterTyped('a', text_model::CharacterStatus::Correct),
            ), (        
                std::time::Duration::from_millis(20), 
                Data::CharacterTyped('g', text_model::CharacterStatus::Wrong),
            )
        ]
    );
}

#[test]
fn writing_data()  {
    let data_capture = DataCapture::<clock::TestClock> {
        clock: Default::default(),
        data: vec![
            (        
                std::time::Duration::from_millis(0),
                Data::CharacterTyped('y', text_model::CharacterStatus::Correct,),
            ), (        
                std::time::Duration::from_millis(10),
                Data::CharacterTyped('o', text_model::CharacterStatus::Correct,),
            ), ( 
                std::time::Duration::from_millis(20),
                Data::Backspace
            )
        ],
    };
    
    let test_db = sled::open("test_db").unwrap();
    data_capture.write_data(&test_db);
}
