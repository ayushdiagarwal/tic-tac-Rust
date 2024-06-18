use crate::board::board_view::Board;

pub enum Player {
    X,
    O,
}

impl Player {
    pub fn switch_player(&mut self) {
        // self actually refers to the current value of the enum which is in use
        match self {
            Player::X => *self = Player::O,
            Player::O => *self = Player::X,
        }
    }

    // maxizingPlayer is X
    // position is the position of the board
    // depth is no of moves left
    pub fn minimax(&mut self, position: [u32; 9], depth: u32, maximizingPlayer: bool) -> i32 {
        let mut temp_board = Board {
            board: position,
            cur_player: if maximizingPlayer {
                Player::X
            } else {
                Player::O
            },
        };

        if depth == 0 || temp_board.checks_winner() {
            return temp_board.evaluate();
        }
        if maximizingPlayer {
            let mut maxEval: i32 = -100;
            for i in 0..9 {
                if temp_board.board[i] == 0 {
                    temp_board.board[i] = 1;
                    let eval = self.minimax(temp_board.board, depth - 1, false);
                    maxEval = std::cmp::max(maxEval, eval);
                    temp_board.board[i] = 0;
                }
            }
            return maxEval;
        } else {
            let mut minEval: i32 = 100;
            for i in 0..9 {
                if temp_board.board[i] == 0 {
                    temp_board.board[i] = 2;
                    let eval = self.minimax(temp_board.board, depth - 1, true);
                    minEval = std::cmp::min(minEval, eval);

                    temp_board.board[i] = 0;
                }
            }
            return minEval;
        }
    }

    pub fn find_best_move(&mut self, position: [u32; 9]) -> usize {
        let mut best_move = None;
        let mut best_value = i32::MAX; // Start with the highest possible value for minimizing

        for i in 0..9 {
            let mut temp_board = Board {
                board: position,
                cur_player: Player::O,
            };
            if temp_board.board[i] == 0 {
                temp_board.make_move(i as u32);
                // temp_board.board[i] = 2; // O is the AI player
                let mut depth = 0; // depth should be the number of moves that is left on the board
                for i in 0..9 {
                    if temp_board.board[i] == 0 {
                        depth += 1;
                    }
                }
                let move_value = self.minimax(temp_board.board, depth, true); // Maximizing player's turn next (X)
                temp_board.board[i] = 0;
                if move_value < best_value {
                    best_value = move_value;
                    best_move = Some(i);
                }
            }
        }
        best_move.unwrap()
    }
}
