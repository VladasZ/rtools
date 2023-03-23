pub trait Selectable {
    fn is_selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);
    fn select(&mut self) {
        self.set_selected(true)
    }
    fn unselect(&mut self) {
        self.set_selected(false)
    }
}
