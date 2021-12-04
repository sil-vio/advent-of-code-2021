use std::collections::HashMap;


#[derive(Debug)]
pub struct BingoBoard {
    map: HashMap<u16, CellStats>,
    row_hit: Vec<u16>,
    column_hit: Vec<u16>,
    winner: bool
}

#[derive(Debug)]
pub struct CellStats {
    x: u16,
    y: u16,
    marked: bool,
}



impl BingoBoard {
    pub fn new() -> BingoBoard {
        BingoBoard {
            map: HashMap::new(),
            row_hit: vec![0,0,0,0,0],
            column_hit: vec![0,0,0,0,0],
            winner: false
        }
    }

    pub fn is_winner(&self) -> bool {
    	self.winner
    }

    pub fn create_cell(&mut self, value: u16, x: u16, y: u16) {
		self.map.insert(value, CellStats{
                x,
                y,
                marked: false,
            });
    }

    pub fn mark_number(&mut self, number:u16 ) -> Option<u32> {
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

    pub fn is_empty(&self) -> bool {
    	self.map.is_empty()
    }
}