use std::boxed::Box;
use super::State;
pub struct Finished;

impl<B> super::State<B> for Finished 
where B: tui::backend::Backend {
    fn handle_event(&mut self, _event: crossterm::event::Event) -> Box<dyn State<B>>  {
        Box::new(Finished)
    }
    fn terminate(&self) -> bool { true }
}