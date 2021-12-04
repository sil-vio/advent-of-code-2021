use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;

fn main() {
    let bingo_game = read_bingo_game("input").expect("Could not load lines");
    bingo_game.play();
    println!("bingoGame: {:?}", bingo_game);

}

fn read_bingo_game(filename: impl AsRef<Path>) -> io::Result<BingoGame> {
    let mut bingo_game = BingoGame{
        boards: Vec::new(),
        number: Vec::new(),
    };
    if let Ok(lines) = read_lines(filename) {
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
                        bingo_board.map.insert(board_row[i], BoardCoordinate{
                            x: i as u16,
                            y: y
                        });
                    }
                    y += 1;
                }
            }
        }
        bingo_game.boards.push(bingo_board);
    }
    Ok(bingo_game)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct BingoBoard {
    map: HashMap<u16, BoardCoordinate>,
    row_hit: Vec<u16>,
    column_hit: Vec<u16>,
    winner: bool
}

#[derive(Debug)]
struct BoardCoordinate {
    x: u16,
    y: u16,
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
            row_hit: Vec::new(),
            column_hit: Vec::new(),
            winner: false
        }
    }

    fn mark_number(&mut self, number:u16 ) -> bool {
        if let Some(cordinate) = self.map.get(&number) {
            self.row_hit[cordinate.x as usize] += self.row_hit[cordinate.x as usize];
        }
        true
    }
}

impl BingoGame {
    fn play(&self) -> u16 {
        for i in 0..self.number.len() {
            for ii in 0..self.boards.len() {
                let var_name: Vec<BingoBoard> = AsMut::as_mut(&mut self.boards);
                var_name[ii].mark_number(self.number[i]);
            }
        }
        16u16
    }
}