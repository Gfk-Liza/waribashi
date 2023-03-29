
const FIRST_SIDE_VALUE: usize = 0;

pub struct Turn {
    pub turn: usize,
}
impl Turn {
    pub fn new() -> Self {
        Self {
            turn: FIRST_SIDE_VALUE
        }
    }
}
