
mod hand;
mod both_hands;
mod moves;
mod turn;

use both_hands::BothHands;
use moves::Moves;

use self::turn::Turn;


pub struct GameBoard {
    pub first_side: BothHands,
    pub second_side: BothHands,
    pub turn: Turn,
}
impl GameBoard {
    pub fn new() -> GameBoard {
        Self {
            first_side: BothHands::new(),
            second_side: BothHands::new(),
            turn: Turn::new()
        }
    }

    pub fn action(&mut self, moves: &Moves) {
        let hand = if moves.is_fist_side(&self.turn) { &mut self.first_side } else { &mut self.second_side };

        if moves.is_divided {
            hand.divide(moves);
            return;
        }

        
    }
}
