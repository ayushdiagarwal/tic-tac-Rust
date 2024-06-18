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
        if depth == 0 || self.checks_winner(position) {
            return self.evaluate(position);
        }

        let mut temp_board: [u32; 9] = position;
        if maximizingPlayer {
            let mut maxEval: i32 = -100;
            for i in 0..9 {
                if temp_board[i] == 0 {
                    temp_board[i] = 1;
                    let eval = self.minimax(temp_board, depth - 1, false);
                    maxEval = std::cmp::max(maxEval, eval);
                    temp_board[i] = 0;
                }
            }
            return maxEval;
        } else {
            let mut minEval: i32 = 100;
            for i in 0..9 {
                if temp_board[i] == 0 {
                    temp_board[i] = 2;
                    let eval = self.minimax(temp_board, depth - 1, true);
                    minEval = std::cmp::min(minEval, eval);
                    temp_board[i] = 0;
                }
            }
            return minEval;
        }
    }

    pub fn checks_winner(&mut self, position: [u32; 9]) -> bool {
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
            let mut x_count = 0;
            let mut o_count = 0;

            for index in condition {
                if position[index as usize] == 1 {
                    x_count += 1;
                } else if position[index as usize] == 2 {
                    o_count += 1;
                }
            }
            if x_count == 3 {
                return true;
            } else if o_count == 3 {
                return true;
            }
        }
        return false;
    }

    pub fn evaluate(&mut self, position: [u32; 9]) -> i32 {
        // Evaluate the board
        // If X wins, return 1
        // If O wins, return -1
        // If draw, return 0

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
            let mut x_count = 0;
            let mut o_count = 0;

            for index in condition {
                if position[index as usize] == 1 {
                    x_count += 1;
                } else if position[index as usize] == 2 {
                    o_count += 1;
                }
            }

            if x_count == 3 {
                return 1;
            } else if o_count == 3 {
                return -1;
            }
        }
        return 0;
    }

    pub fn find_best_move(&mut self, position: [u32; 9]) -> usize {
        let mut best_move = None;
        let mut best_value = 100;

        let mut temp_board = position;
        for i in 0..9 {
            if temp_board[i] == 0 {
                temp_board[i] = 2; // O is the AI player
                                   // it is now X's turn to play, so we need to set maximizingPlayer = True;
                let move_value = self.minimax(temp_board, 9, true);
                println!("move value: {}", move_value);
                temp_board[i] = 0;
                if move_value < best_value {
                    best_value = move_value;
                    best_move = Some(i);
                }
            }
        }
        best_move.unwrap()
    }
}
