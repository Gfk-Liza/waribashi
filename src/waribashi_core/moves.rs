
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
    pub add_value: HandValueType,
    pub is_destination_left_hand: bool,
    pub is_source_left_hand: bool,
    pub is_divided: bool,
}
impl Moves {
    pub fn new() -> Self {
        Self {
            add_value: DEFAULT_HAND_VALUE,
            is_destination_left_hand: DEFAULT_HAND,
            is_source_left_hand: DEFAULT_HAND,
            is_divided: DEFAULT_MOVE_MODE
        }
    }

    pub fn get_destination(&self, turn: &Turn) -> Turn {
        turn.turn ^ !self.is_divided

        // true ^ !true == true
        //true ^ !false == false
        //false ^ !true == false
        //false ^ !false == true
    }
}
