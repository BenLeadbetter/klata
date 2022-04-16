pub trait State<B>
where
    B: tui::backend::Backend,
{
    fn handle_event(&mut self, event: crossterm::event::Event) -> std::boxed::Box<dyn State<B>>;
    fn terminate(&self) -> bool {
        false
    }
    fn ui(&self, _: &mut tui::Frame<B>) {}
}
