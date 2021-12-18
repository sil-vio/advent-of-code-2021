use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
mod comm;

fn get_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let lines: Vec<String> = BufReader::new(File::open(filename)?)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    Ok(lines)
}


fn main() {
    let time = Instant::now();
    part1(get_file("input").unwrap());
    println!("part1 ns {}", time.elapsed().as_nanos());
    let time = Instant::now();
    part2(get_file("input").unwrap());
    println!("part2 ns {}", time.elapsed().as_nanos());
}


pub fn part1(lines: Vec<String>) -> i64 {
    let magnitude = lines.iter().map(|x| comm::Comm::new(x.clone())).fold(comm::Comm::None, |tot, x| tot + x).magnitude();    
    println!("day 18a result: {}", magnitude);
    magnitude
}


pub fn part2(lines: Vec<String>) -> i64 {
    let data: Vec<comm::Comm> = lines.iter().map(|x| comm::Comm::new(x.clone())).collect();
    let result = (0..data.len())
        .map(|i| (0..data.len())
            .filter(|&j| j != i)
            .map(|j| (data[i].clone() + data[j].clone()).magnitude()).max().unwrap()).max().unwrap();
    println!("day 18b result: {}", result);
    result
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part1() {
        let lines = get_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(part1(lines), 4140);
    }

   #[test]
    fn test_part2() {
        let lines = get_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(part2(lines), 3993);
    }
}
