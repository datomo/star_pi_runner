use crate::workflow::BlueprintBlock;

pub(crate) trait Block {
    fn new(block: BlueprintBlock) -> Self;
}


trait Action {
    fn run();
}


pub(crate) trait Trigger {
    /// returns
    fn check_status(&self) -> bool;

    fn event_loop(&mut self);
}




