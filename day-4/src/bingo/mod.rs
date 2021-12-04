use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod board;
use board::BingoBoard;


#[derive(Debug)]
pub struct BingoGame {
    number: Vec<u16>,
    boards: Vec<BingoBoard>
}


impl BingoGame {

    pub fn new_from_file(filename: impl AsRef<Path>) -> BingoGame {
        let mut bingo_game = BingoGame{
            boards: Vec::new(),
            number: Vec::new(),
        };
        let file = File::open(filename);
        let lines = io::BufReader::new(file.unwrap()).lines();
        let mut bingo_board = BingoBoard::new();
        let mut y: u16 = 0;
        for line in lines {
            if let Ok(value) = line {
                if value.len() == 0 {
                    if !bingo_board.is_empty() {
                        bingo_game.boards.push(bingo_board);
                    }
                    bingo_board = BingoBoard::new();
                    y = 0;
                } else if value.len() > 15 {
                    // input data
                    bingo_game.number = value.split(',').map(|e| e.parse::<u16>().unwrap()).collect::<Vec<u16>>();
                } else {
                    // board data
                    let board_row = value.split(' ').filter(|e| !e.is_empty()).map(|e|e.parse::<u16>().unwrap()).collect::<Vec<u16>>();
                    for i in 0..board_row.len() {
                        bingo_board.create_cell(board_row[i], i as u16, y);
                    }
                    y += 1;
                }
            }
        }
        bingo_game.boards.push(bingo_board);
        println!("bingo game create succesfully!");
        bingo_game
    }
    

    pub fn play(&mut self) -> (u32, u32) {
        let mut first_result = 0;
        let mut last_result = 0;
        let mut board_winner_counter = 0;
        println!("Start extraction ...");
        for i in 0..self.number.len() {
            println!{"{} ", self.number[i]};
            for ii in 0..self.boards.len() {
                if !self.boards[ii].is_winner() {
                    if let Some(board_result) = self.boards[ii].mark_number(self.number[i]) {
                        board_winner_counter += 1;
                        println!("Bingo! We have a winner!");
                        if first_result == 0 {
                            first_result = board_result;
                        } else {
                            last_result = board_result;
                        }

                    }
                }
            }
            if board_winner_counter == self.boards.len() {
                break;                            
            }
        }
        (first_result, last_result)
    }
}