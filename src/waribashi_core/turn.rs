use super::rule::Rule;


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

    pub fn next(&mut self, rule: &Rule) {
        self.turn += 1;
        if self.turn >= rule.players_num {
            self.turn = FIRST_SIDE_VALUE;
        }
    }
}
