mod board;
use board::board_view;

fn main() {
    // 1 -> O
    // 2 -> X

    let mut board: [[u32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    let mut grid = board_view::Board {
        board, // shorthand intialization
    };

    grid.prints_board();
}
