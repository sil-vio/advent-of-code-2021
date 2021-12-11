use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: u64,
}

fn main() {
    let mut input = lines_from_file("input").unwrap();
    let now = Instant::now();
    let (p1, p2) = part1(&mut input);
    println!(
        "part 1 result: {}, part2 result {} in ns {}",
        p1,
        p2,
        now.elapsed().as_nanos()
    );
}

fn part1(input: &mut Vec<Vec<u64>>) -> (u64, usize) {
    let mut flash_counter = 0;
    let mut step = 1;
    loop {
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
                if input[y][x] == 10 {
                    flash_point(input, x, y);
                }
            }
        }
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] > 9 {
                    input[y][x] = 0;
                    if step <= 100 {
                        flash_counter += 1;
                    }
                }
            }
        }
        let mut all_zero = true;
        'outer: for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] != 0 {
                    all_zero = false;
                    break 'outer;
                }
            }
        }
        if all_zero {
            break;
        }
        step += 1;
    }
    (flash_counter, step)
}

fn flash_point(input: &mut Vec<Vec<u64>>, x: usize, y: usize) {
    let mut y_near = Vec::new();
    if y > 0 {
        y_near.push(y - 1);
    }
    if y < input.len() - 1 {
        y_near.push(y + 1);
    }
    y_near.push(y);
    let mut x_near = Vec::new();
    if x > 0 {
        x_near.push(x - 1);
    }
    if x < input[y].len() - 1 {
        x_near.push(x + 1);
    }
    x_near.push(x);
    for near_y in &y_near {
        for near_x in &x_near {
            if !(*near_x == x && *near_y == y) && input[*near_y][*near_x] < 10 {
                input[*near_y][*near_x] += 1;
                if input[*near_y][*near_x] == 10 {
                    flash_point(input, *near_x, *near_y);
                }
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let mut lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        for line in &lines {
            assert_eq!(line.len(), 10);
        }
        let (p1, p2) = part1(&mut lines);
        assert_eq!(p1, 1656);
        assert_eq!(p2, 195)
    }
}
