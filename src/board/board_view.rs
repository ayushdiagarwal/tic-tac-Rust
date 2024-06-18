use crate::game::player::Player;

pub struct Board {
    pub board: [u32; 9],
    pub cur_player: Player,
}

impl Board {
    // prints the board
    pub fn prints_board(&mut self) {
        for row in 0..3 {
            for col in 0..3 {
                print!("{} ", self.to_player(self.board[row * 3 + col]));
            }
            println!()
        }
    }

    pub fn to_player(&mut self, val: u32) -> char {
        match val {
            0 => '_',
            1 => 'X',
            2 => 'O',
            _ => ' ',
        }
    }

    pub fn player_to_u32(&mut self) -> u32 {
        match self.cur_player {
            Player::X => 1,
            Player::O => 2,
        }
    }

    // makes a move
    pub fn make_move(&mut self, pos: u32) {
        if self.board[pos as usize] == 0 {
            self.board[pos as usize] = self.player_to_u32();

            if self.checks_winner() {
                self.prints_board();
                println!("Player {} wins!", self.player_to_u32());
                return;
            } else {
                if !self.checks_draw() {
                    println!("\n-----\n");
                    self.cur_player.switch_player();
                }
            }
        } else {
            println!("Invalid move");
            // returns to the same player
        }
    }

    pub fn checks_draw(&mut self) -> bool {
        for i in 0..9 {
            if self.board[i] == 0 {
                return false;
            }
        }
        println!("It's a draw!");
        return true;
    }

    pub fn checks_winner(&mut self) -> bool {
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

            for index in condition {
                if self.board[index as usize] == self.player_to_u32() {
                    count += 1;
                }
            }

            if count == 3 {
                return true;
                // the cur_player would be the winner
            }
        }
        return false;
    }
}
