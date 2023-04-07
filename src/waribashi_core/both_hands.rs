
use super::{
    hand::Hand,
    moves::Moves, hand_index::HandIndex
};


#[derive(Clone, Copy)]
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

    pub fn get_hand(&mut self, hand_index: &HandIndex) -> &mut Hand {
        if hand_index.is_left_hand {
            return &mut self.left_hand;
        }
        &mut self.right_hand
    }

    pub fn print_hand(&self) {
        let print_space = || { print!("  "); };

        print_space();
        println!("left hand: {}", self.left_hand.value);
        print_space();
        println!("right hand: {}", self.right_hand.value);
    }
}
