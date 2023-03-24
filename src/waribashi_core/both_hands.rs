
use super::{
    hand::Hand,
    moves::Moves
};

pub struct BothHands {
    pub left_hand: Hand,
    pub right_hand: Hand,
}
impl BothHands {
    pub fn new() -> Self {
        Self {
            left_hand: Hand::new(),
            right_hand: Hand::new()
        }
    }

    pub fn divide(&mut self, moves: &Moves) {
        if moves.is_source_left_hand {
            self.right_hand.value += moves.add_value;
            self.left_hand.value -= moves.add_value;
        }
        else {
            self.left_hand.value += moves.add_value;
            self.right_hand.value -= moves.add_value;
        }
    }

    pub fn add(&mut self, moves: &Moves) {
        
    }
}

