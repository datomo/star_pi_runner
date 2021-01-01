trait Action {
    fn run();
}


pub(crate) trait Trigger {
    /// returns
    fn check_status(&self) -> bool;

    fn event_loop(&mut self);
}



