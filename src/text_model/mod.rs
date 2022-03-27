#[cfg(test)]
mod tests;

mod data;
mod text_model;

pub use text_model::Event;
pub use text_model::Character;
pub use text_model::CharacterStatus;
pub use text_model::Observer;
pub use text_model::TextModel;
