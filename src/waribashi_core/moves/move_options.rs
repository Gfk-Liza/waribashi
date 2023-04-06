


pub struct MoveOptions {
    pub is_attack: bool,
    pub enable_divide: bool,
    pub is_changing_positive_and_negative: bool,
}
impl MoveOptions {
    // 初期値はすべてfalse
    pub fn new() -> Self {
        Self {
            is_attack: false,
            enable_divide: false,
            is_changing_positive_and_negative: false
        }
    }
}
