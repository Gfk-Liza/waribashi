
use super::Rule;


pub type HandValueType = i8;

pub const DEFAULT_HAND_VALUE: HandValueType = 0;


#[derive(Clone, Copy)]
pub struct Hand {
    pub value: HandValueType,
}
impl Hand {
    pub fn new() -> Self {
        Self {
            value: DEFAULT_HAND_VALUE
        }
    }

    pub fn add(&mut self, add_value: &HandValueType, max_hand_value: &HandValueType) {
        self.value += add_value;
        if self.value.abs() > *max_hand_value {
            self.value = 0;
        }
    }

    pub fn to_be_negative(&mut self) {
        self.value *= -1;
    }
}
