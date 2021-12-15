use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::usize;

fn main() {
    let tile = lines_from_file("input").unwrap();
    println!("min dist to exit : {}", part1(&tile,1));
    let complete_map:Vec<Vec<u64>> = create_map(&tile);
    println!("min dist to exit : {}", part1(&complete_map, 1));

}

fn part1(input: &Vec<Vec<u64>>, map_size: usize) -> u64 {
    // let mut dist: Vec<Vec<u64>> = vec![vec![u64::MAX; input[0].len() * map_size]; input.len() * map_size];
    let mut map: HashMap<(usize, usize), u64> = HashMap::new();
    map.insert((0, 0), input[0][0]);
    let  (mut y, mut x): (usize, usize) = (0,0);
    let max_x = input[0].len();
    let max_y = input.len();
    while y < max_y * map_size {
        while x < max_x * map_size{
            let dist_to_point = map.get(&(y, x)).unwrap().clone();
            for (ny, nx) in get_neighbours(y, x, max_y, max_x, map_size) {
                let dist_to_neighbour = map.entry((ny, nx)).or_insert(u64::MAX);
                let new_dist_neighbour: u64 = dist_to_point + get_value_from_map(input, ny, nx);
                if *dist_to_neighbour > new_dist_neighbour {
                    *dist_to_neighbour = new_dist_neighbour;
                    if ny < y || nx < x {
                        y = y.min(ny);
                        x = x.min(nx - 1);
                    }
                } 
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    map.get(&(max_y* map_size -1 , max_x* map_size -1)).unwrap() - input[0][0]
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

fn create_map(tile: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut complete_map = vec![vec![0; tile[0].len() * 5]; tile.len() * 5];
    for y in 0..complete_map.len() {
        for x in 0..complete_map[y].len() {
            complete_map[y][x] = get_value_from_map(tile, y, x);
        }
    }
    complete_map
}

fn get_value_from_map(tile:  &Vec<Vec<u64>>, y:usize, x:usize) -> u64 {
    let mod_y = y % tile.len();
    let mod_x = x % tile[0].len();
    let plus_y = if y >= tile.len() { y / tile.len() } else { 0 };
    let plus_x = if x >= tile[0].len() { x / tile.len() } else { 0 };          
    let new_value = tile[mod_y][mod_x] + plus_x as u64 + plus_y as u64;
    return  if new_value > 9 { new_value % 9 } else { new_value };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let tile = lines_from_file("test_input").unwrap();
        let complete_map:Vec<Vec<u64>> = create_map(&tile);
        for row in &complete_map {
            println!("{:?}", row);
        }
        assert_eq!(tile.len(), 10);
        for line in &tile {
            assert_eq!(line.len(), 10);
        }
        assert_eq!(tile[2][2], 3);
        assert_eq!(part1(&tile, 1), 40);
        assert_eq!(part1(&complete_map, 1), 315);
        assert_eq!(part1(&tile, 5), 315);

    }

   
}
