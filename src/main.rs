
mod waribashi_core;

use waribashi_core::{
    GameBoard,
    rule::Rule
};

fn main() {
    let mut rule = Rule::new();
    let mut board = GameBoard::new(&rule);
    board.print_game_board();
}
