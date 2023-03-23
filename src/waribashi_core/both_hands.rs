
use super::hand::Hand;

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
}

