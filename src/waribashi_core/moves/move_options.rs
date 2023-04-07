


pub struct MoveOptions {
    pub is_attack: bool,
    pub is_division: bool,
    pub is_changing_positive_and_negative: bool,
}
impl MoveOptions {
    // 初期値はすべてfalse
    pub fn new() -> Self {
        Self {
            is_attack: false,
            is_division: false,
            is_changing_positive_and_negative: false
        }
    }
}
