use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

struct Point {
    x: usize,
    y: usize,
}


fn main() {
    let input = lines_from_file("input").unwrap();
    println!("part 1 result: {:?} ", part1(&input));
    println!("part 2 result: {:?} ", part2(&input));
}

fn part1(input: &Vec<Vec<u64>>) -> u64 {
    let mut counter = 0;
    let mut low_point: Vec<Point> = Vec::new();
    for y in 0..input.len() {
        let mut y_near = Vec::new();
        if y > 0 {
            y_near.push(y-1);
        }
        if y < input.len() - 1 {
            y_near.push(y + 1);
        }
        y_near.push(y);
        for x in 0..input[y].len() {
            let mut x_near = Vec::new();
            if x > 0 {
                x_near.push(x-1);
            }
            if x < input[y].len() -1 {
                x_near.push(x + 1);
            }
            x_near.push(x);
            let mut is_min = true;
            for near_y in &y_near {
                for near_x in &x_near {
                    if  ((*near_x != x || *near_y != y) && (x == *near_x || y == *near_y))  && input[y][x] >= input[*near_y][*near_x]  {
                        is_min = false;
                    }
                }
            }
            if is_min == true {
                println!(" found min in [{},{}] value {} ",y,x,input[y][x]);
                low_point.push(Point{y,x});
                counter += input[y][x] + 1;
            }
        }
    }
   
    return counter;
}

fn part2(input: &Vec<Vec<u64>>) -> u64 {
    let mut output_sum = 0;
    
    output_sum
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<u64>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| s.unwrap().chars().map(|c| c.to_string().parse::<u64>().unwrap()).collect())
        .collect::<Vec<Vec<u64>>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 5);
        for line in &lines {
            assert_eq!(line.len(), 10);
        }
        assert_eq!(lines[2][2], 5);
        assert_eq!(part1(&lines), 15);      
    }

}
