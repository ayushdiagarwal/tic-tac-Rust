pub enum Player {
    X,
    O,
}

impl Player {
    pub fn switch_player(&mut self) {
        match self {
            Player::X => *self = Player::O,
            Player::O => *self = Player::X,
        }
    }
}
