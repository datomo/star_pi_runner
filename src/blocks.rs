trait Action {
    fn run();
}


pub(crate) trait Trigger {
    /// returns
    fn check_status(&self) -> bool;

    fn event_loop(&mut self);
}


pub struct Command {
    pub(crate) sender_id: i32,
    pub(crate) message: String,
}

