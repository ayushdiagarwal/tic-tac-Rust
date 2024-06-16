mod board;
use board::board_view;

mod game;
use game::player::Player;

use std::io;

fn main() {
    let board: [u32; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let cur_player = Player::X;

    // when I'm passing board and cur_player, I'm passing a copy of it, it would be better to pass the reference to it
    let mut grid = board_view::Board {
        board, // shorthand intialization
        cur_player,
    };

    while !grid.checks_winner() && grid.checks_draw() {
        grid.prints_board();

        println!("Enter Your Move: ");
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Enter something nice");

        let inp_move: u32 = inp.trim().parse().expect("Enter some int ya cunt");

        grid.make_move(inp_move - 1);
    }
}
