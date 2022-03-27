use crate::text_model;
use serde::{ Serialize, Deserialize };

mod clock;
#[cfg(test)]
mod tests;

pub use clock::RealClock;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Data {
    CharacterTyped(char, text_model::CharacterStatus),
    Backspace,
}

pub struct DataCapture<T>
where 
    T: clock::Clock + std::default::Default {
    clock: T,
    data: Vec<(std::time::Duration, Data)>,
}

impl<T> DataCapture<T>
where 
    T: clock::Clock + std::default::Default {
    pub fn new() -> Self {
        DataCapture {
            clock: Default::default(),
            data: Vec::new(),
        }
    }
    
    pub fn write_data(&self, db: &sled::Db) {
        for datum in self.data.iter() {
            db.insert(
                datum.0.as_millis().to_be_bytes(), 
                sled::IVec::from(bincode::serialize(&datum.1).unwrap())
            ).expect("value inserted");
        }
    }
}

impl<T> text_model::Observer for DataCapture<T> 
where T: clock::Clock + std::default::Default {
    fn notify(&mut self, event: &text_model::Event) {
        match event {
            text_model::Event::Backspace => {
                self.data.push((self.clock.elapsed(),Data::Backspace));
            },
            text_model::Event::Type((c, s)) => {
                self.data.push((self.clock.elapsed(), Data::CharacterTyped(*c, *s)));
            },
        }
    }
}