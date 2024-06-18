mod board;
use board::board_view::Board;

mod game;
use game::player::Player;

use std::io;

fn main() {
    let mut grid = Board {
        board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
        cur_player: Player::O,
    };

    // For against the comp
    while !grid.checks_winner() && !grid.checks_draw() {
        grid.prints_board();

        match grid.cur_player {
            Player::X => {
                println!("Enter Your Move: ");
                let mut inp = String::new();

                io::stdin()
                    .read_line(&mut inp)
                    .expect("Enter something nice");

                let inp_move: u32 = inp.trim().parse().expect("Enter some int ya cunt");
                grid.make_move(inp_move - 1);
            }
            Player::O => {
                println!("-------");
                let ai_move = grid.cur_player.find_best_move(grid.board);
                grid.make_move(ai_move as u32);
            }
        }
    }
    if grid.checks_winner() {
        grid.prints_board();
        println!("Player {} WINS!", grid.player_to_u32());
    }
}
