
use super::{
    hand::{
        HandValueType,
        DEFAULT_HAND_VALUE
    },
    turn::Turn
};

const DEFAULT_HAND: bool = true;
const DEFAULT_MOVE_MODE: bool = false;

pub struct Moves {
    pub turn: Turn,
    pub add_value: HandValueType,
    pub is_left_hand: bool,
    pub is_divided: bool,
}
impl Moves {
    pub fn new() -> Self {
        Self {
            turn: Turn::new(),
            add_value: DEFAULT_HAND_VALUE,
            is_left_hand: DEFAULT_HAND,
            is_divided: DEFAULT_MOVE_MODE
        }
    }
}
