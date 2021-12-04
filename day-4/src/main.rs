use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let mut bingo_game = BingoGame::new_from_file("input");
    let (first_board, last_board) = bingo_game.play();
    println!("Bingo result, first_board: {:?},  last_board: {:?}", first_board, last_board);

}

#[derive(Debug)]
struct BingoBoard {
    map: HashMap<u16, CellStats>,
    row_hit: Vec<u16>,
    column_hit: Vec<u16>,
    winner: bool
}

#[derive(Debug)]
struct CellStats {
    x: u16,
    y: u16,
    marked: bool,
}

#[derive(Debug)]
struct BingoGame {
    number: Vec<u16>,
    boards: Vec<BingoBoard>
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            map: HashMap::new(),
            row_hit: vec![0,0,0,0,0],
            column_hit: vec![0,0,0,0,0],
            winner: false
        }
    }

    fn mark_number(&mut self, number:u16 ) -> Option<u32> {
        if let Some(coordinate) = self.map.get(&number) {
            let cell_stats_marked = CellStats {x: coordinate.x, y: coordinate.y, marked: true};
            self.row_hit[coordinate.x as usize] +=  1;
            self.column_hit[coordinate.y as usize] +=  1;
            if self.row_hit[coordinate.x as usize] == 5 || self.column_hit[coordinate.y as usize] == 5 {
                self.map.insert(number, cell_stats_marked);
                let mut sum = 0 as u16;
                 for (value, cel_stats) in self.map.iter() {
                    if !cel_stats.marked {
                        sum += value;
                    }
                }
                self.winner = true;
                return Some(sum as u32 * number as u32);
            } else {
                self.map.insert(number, cell_stats_marked);
            }
        }
        return None;
    }
}

impl BingoGame {

    fn new_from_file(filename: impl AsRef<Path>) -> BingoGame {
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
                    println!("new board ");
                    if bingo_board.map.len() > 0 {
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
                    println!("{:?}", board_row);
                    for i in 0..board_row.len() {
                        bingo_board.map.insert(board_row[i], CellStats{
                            x: i as u16,
                            y,
                            marked: false,
                        });
                    }
                    y += 1;
                }
            }
        }
        bingo_game.boards.push(bingo_board);
        bingo_game
    }
    

    fn play(&mut self) -> (u32, u32) {
        let mut first_result = 0;
        let mut last_result = 0;
        let mut board_winner_counter = 0;
        println!("Start extraction ...");
        for i in 0..self.number.len() {
            println!{"{} ", self.number[i]};
            for ii in 0..self.boards.len() {
                if !self.boards[ii].winner {
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