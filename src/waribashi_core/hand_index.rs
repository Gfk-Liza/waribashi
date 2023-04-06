
const DEFAULT_IS_LEFT_HAND: bool = true;

pub struct HandIndex {
    pub is_left_hand: bool,
}
impl HandIndex {
    pub fn new() -> Self {
        Self { is_left_hand: DEFAULT_IS_LEFT_HAND }
    }
}
