
use super::{
    super::Turn,
    HandIndex
};


pub struct HandAddress {
    pub turn: Turn,
    pub hand_index: HandIndex,
}
impl HandAddress {
    pub fn new() -> Self {
        Self {
            turn: Turn::new(),
            hand_index: HandIndex::new()
        }
    }
}
