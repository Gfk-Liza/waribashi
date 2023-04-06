
mod move_options;
mod address;

use super::{
    hand::HandValueType,
    hand_index::HandIndex
};
use {
    move_options::MoveOptions,
    address::HandAddress
};




pub struct Moves {
    pub add_value: HandValueType,
    pub options: MoveOptions,
    pub starting_point: HandAddress,
    pub destination: HandAddress,
}
impl Moves {
    pub fn new() -> Self {
        Self {
            add_value: 0,
            options: MoveOptions::new(),
            starting_point: HandAddress::new(),
            destination: HandAddress::new()
        }
    }
}
