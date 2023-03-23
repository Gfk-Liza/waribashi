
const FIRST_SIDE_VALUE: bool = true;

pub struct Turn {
    pub turn: bool,
}
impl Turn {
    pub fn new() -> Self {
        Self {
            turn: FIRST_SIDE_VALUE
        }
    }
}
