
const DEFAULT_IS_SUICIDE_POSSIBLE: bool = true;
const DEFAULT_IS_POSSIBLE_TO_COMMIT_SUICIDE_TO_CREATE_A_MAXIMUM: bool = false;


pub struct Suicide {
    pub is_suicide_possible: bool,
    pub is_possible_to_commit_suicide_to_create_a_maximum: bool,
}
impl Suicide {
    pub fn new() -> Self {
        Self {
            is_suicide_possible: DEFAULT_IS_SUICIDE_POSSIBLE,
            is_possible_to_commit_suicide_to_create_a_maximum: DEFAULT_IS_POSSIBLE_TO_COMMIT_SUICIDE_TO_CREATE_A_MAXIMUM
        }
    }
}
