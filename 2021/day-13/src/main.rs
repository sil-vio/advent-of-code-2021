use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use std::num::ParseIntError;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x,y) = s.split_once(',').unwrap_or(("a", "a"));
        Ok(Point{
            x: x.parse::<usize>()?,
            y: y.parse::<usize>()?
        })
    }
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

fn part1(input: &mut Vec<Point>) -> (u64, usize) {
    let mut flash_counter = 0;
    let mut step = 1;
    (flash_counter, step)
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Point>> {
    let file = File::open(filename)?;
    let points: Vec<Point> = io::BufReader::new(file).lines()
        .map(|line| Point::from_str(line.unwrap().as_str()))
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
    .collect();
    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let  points = lines_from_file("test_input").unwrap();
        assert_eq!(points.len(), 18);
    }
}
