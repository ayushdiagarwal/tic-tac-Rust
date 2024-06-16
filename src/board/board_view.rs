// using enum for the current player
// struct Game {player: cur_player, board: board_view}
// make impl board which does all the basic operations
// prints_board, makes_move, changes_player, checks_winner, checks_draw, initializes_board

// this is the file which should contain the board view
//
// when using this function, just pass in the board, and it should work fine

// use a 1-D array instead of a 2-D array
pub struct Board {
    pub board: [[u32; 3]; 3],
}

impl Board {
    // prints the board
    pub fn prints_board(&mut self) {
        for row in 0..3 {
            println!("{:?}", self.board[row]);
        }
    }

    // cur_player needs to passed in as a reference
    pub fn make_move(&mut self, cur_player: u32, pos: u32) {
        // makes a move
        // change the player and the value of

        let row = (pos / 3) as usize;
        let col = (pos % 3) as usize;

        // check if it is empty or not

        if self.board[row][col] == 0 {
            self.board[row][col] = cur_player;

            // call the function to change the player
            self.change_player(cur_player);
        } else {
            println!("Invalid move");
        }
    }

    pub fn change_player(&mut self, cur_player: u32) {
        // I need a reference to the player, instead of the player itself, so I can change its value
    }

    pub fn checks_winner(&mut self, cur_player: u32) -> bool {
        let win_conditions = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for condition in win_conditions {
            let mut count = 0;

            // for index in condition {
            //     if self.board[index as usize] == cur_player {
            //         count += 1;
            //     }
            // }

            if count == 3 {
                return true;
            }
        }
        return false;
    }
}
