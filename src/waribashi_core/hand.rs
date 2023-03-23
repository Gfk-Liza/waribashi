
pub type HandValueType = u8;

pub const DEFAULT_HAND_VALUE: HandValueType = 0;


pub struct Hand {
    pub value: HandValueType,
}
impl Hand {
    pub fn new() -> Self {
        Self {
            value: DEFAULT_HAND_VALUE
        }
    }
}
