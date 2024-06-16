// arguments would be the board and the current player
fn checkWinner() -> bool {
    // check all possible conditions for the winner,
    true
}

fn main() {
    // 1 -> O
    // 2 -> X

    let mut board: [[u32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    // I will use referencing to print the board because it sounds more cool heheeh

    for row in 0..3 {
        println!("{:?}", board[row]);
    }
}
