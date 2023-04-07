
mod hand;
mod both_hands;
mod moves;
mod turn;
pub mod rule;
mod hand_index;

use both_hands::BothHands;
use moves::{
    Moves,
    address::HandAddress
};
use rule::Rule;
use self::{
    turn::Turn,
    hand::Hand
};


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
            || { index += 1; new_rule.default_hands[index - 1] }
        );

        Self {
            players_hands: new_hands,
            turn: Turn::new(),
            rule: new_rule.clone()
        }
    }

    pub fn action(&mut self, moves: &Moves) {
        let max_hand_value = self.rule.max_hand_value.clone();
        if moves.options.is_division {
            let tmp_hand = self.get_hand(
                &moves.starting_point
            );
            tmp_hand.add(
                & -moves.add_value,
                &max_hand_value
            );
        }
        {
            let tmp_hand = self.get_hand(
                &moves.destination
            );
            tmp_hand.add(
                &moves.add_value,
                &max_hand_value
            );
        }
    }

    fn get_hand(&mut self, address: &HandAddress) -> &mut Hand {
        self.players_hands[address.turn.turn].get_hand(&address.hand_index)
    }

    pub fn print_game_board(&self) {
        for (index, hand) in self.players_hands.iter().enumerate() {
            println!("player's index: {}", index);
            hand.print_hand();
        }
    }
}
