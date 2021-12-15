use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use std::usize;

fn main() {
    let tile = lines_from_file("input").unwrap();
    let p1_duration = Instant::now();
    println!("min dist to exit : {} in ns {}", part2(&tile,1), p1_duration.elapsed().as_nanos());
    let p2_duration = Instant::now();
    println!("min dist to exit : {} in ns {}", part2(&tile, 5), p2_duration.elapsed().as_nanos());

}

fn part2(input: &Vec<Vec<u64>>, map_size: usize) -> u64 {
    let mut map: HashMap<(usize, usize), u64> = HashMap::new();
    let tile_x = input[0].len();
    let tile_y = input.len();
    let mut basket: BinaryHeap<Reverse<(u64, (usize, usize))>> = BinaryHeap::new();
    basket.push(Reverse((0, (0,0))));
    let mut min_cost = u64::MAX;
    while let Some(Reverse((cost,  (y, x)))) = basket.pop() {
        if x == tile_x*map_size -1 && y == tile_y*map_size -1 {
            min_cost = cost;
        }
        for (ny, nx) in get_neighbours(y, x, tile_y, tile_x, map_size) {
            let dist_to_neighbour = map.entry((ny, nx)).or_insert(u64::MAX);
            let new_dist_neighbour: u64 = cost + get_value_from_map(input, ny, nx);
            if *dist_to_neighbour > new_dist_neighbour {
                *dist_to_neighbour = new_dist_neighbour;
                basket.push(Reverse((new_dist_neighbour, (ny, nx))));
            } 
        }
    }
    min_cost
}


fn get_neighbours(y: usize, x: usize, len_y: usize, len_x: usize, map_size: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if y > 0 {
        neighbours.push((y - 1, x));
    }
    if y < (len_y * map_size) - 1 {
        neighbours.push((y + 1, x));
    }
    if x > 0 {
        neighbours.push((y, x - 1));
    }
    if x < (len_x * map_size) - 1{
        neighbours.push((y, x + 1));
    }
    neighbours
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<u64>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>())
}

fn get_value_from_map(tile:  &Vec<Vec<u64>>, y:usize, x:usize) -> u64 {
    let plus_y = if y >= tile.len() { y / tile.len() } else { 0 };
    let plus_x = if x >= tile[0].len() { x / tile.len() } else { 0 };          
    let new_value = tile[y % tile.len()][x % tile[0].len()] + plus_x as u64 + plus_y as u64;
    return  if new_value > 9 { new_value % 9 } else { new_value };
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_parsing_part2() {
        let tile = lines_from_file("test_input").unwrap();
        assert_eq!(part2(&tile, 1), 40);
        assert_eq!(part2(&tile, 5), 315);
    }

   
}
