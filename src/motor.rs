use crate::blocks::Block;
use crate::workflow::BlueprintBlock;

pub(crate) struct Motor {
    pin: Vec<i32>,
}

impl Block for Motor {
    fn new(block: BlueprintBlock) -> Self {
        Motor { pin: block.pins }
    }
}