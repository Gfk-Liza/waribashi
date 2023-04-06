
mod hand;
mod both_hands;
mod moves;
mod turn;
mod rule;
mod hand_index;

use both_hands::BothHands;
use moves::Moves;
use rule::Rule;
use self::turn::Turn;



pub struct GameBoard {
    pub players_hands: Vec<BothHands>,
    pub turn: Turn,
    pub rule: Rule,
}
impl GameBoard {
    pub fn new(new_rule: &Rule) -> GameBoard {
        let mut new_hands: Vec<BothHands> = Vec::new();
        let mut index: usize = 0;
        new_hands.resize_with(
            new_rule.players_num,
            || { index += 1; new_rule.default_hands[index] }
        );

        Self {
            players_hands: new_hands,
            turn: Turn::new(),
            rule: new_rule.clone()
        }
    }

    pub fn action(&mut self, moves: &Moves) {

    }
}
