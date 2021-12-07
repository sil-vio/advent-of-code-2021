use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn main() {
    let mut numbers = lines_from_file("input").unwrap();
    let median = median(&mut numbers);
    println!("{:?}", median);
    let sum: i32 = numbers.iter().map(|e| (e - median).abs()).sum();
    println!("{:?}", sum);

    let media = 463 as i32;
        let sum: i32 = numbers.iter().map(|e| (e - media).abs()).sum();

    println!("{:?}", sum);
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<i32>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .next()
        .unwrap().unwrap()
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}