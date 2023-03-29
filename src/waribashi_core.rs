
mod hand;
mod both_hands;
mod moves;
mod turn;
mod rule;

use both_hands::BothHands;
use moves::Moves;
use rule::Rule;
use self::turn::Turn;
use std::vec;

pub struct GameBoard {
    pub playres_hands: Vec<BothHands>,
    pub turn: Turn,
    pub rule: Rule,
}
impl GameBoard {
    pub fn new(new_rule: &Rule) -> GameBoard {
        let new_hands: Vec<BothHands> = Vec::new();
        let index: usize = 0;
        new_hands.resize_with(
            new_rule.players_num,
            || { index += 1; new_rule.default_hands[index] }
        );
        
        Self {
            playres_hands: new_hands,
            turn: Turn::new(),
            rule: *new_rule.clone()
        }
    }

    pub fn action(&mut self, moves: &Moves) {
        let hand = &mut self.playres_hands[moves.get_destination(&self.turn).turn];

        if moves.is_divided {
            hand.divide(moves);
            return;
        }


    }
}
