mod bingo;

fn main() {
    let mut bingo_game = bingo::BingoGame::new_from_file("input");
    let (first_board, last_board) = bingo_game.play();
    println!("Bingo result, first_board: {:?},  last_board: {:?}", first_board, last_board);
}
