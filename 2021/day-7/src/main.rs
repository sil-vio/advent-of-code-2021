use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}


fn average(numbers: &[i32]) -> f32 {
    (numbers.iter().sum::<i32>() as f32) / numbers.len() as f32
}

fn summation(n: i32) -> i32 {
    n * (n +1)/2
}

fn main() {
    let mut numbers = lines_from_file("input").unwrap();
    let median = median(&mut numbers);
    let sum: i32 = numbers.iter().map(|e| (e - median).abs()).sum();
    println!("part 1 median {:?}, sum of difference {:?}", median, sum);

    let average = average(& numbers);
    let floor_avarage = average.floor() as i32;
    let ceil_avarage = average.ceil() as i32;
    let sum_floor: i32 = numbers.iter().map(|e| summation((e - floor_avarage).abs())).sum();
    let sum_ceil: i32 = numbers.iter().map(|e| summation((e - ceil_avarage).abs())).sum();

    println!("part 2 average {:?}, sum of difference {:?}", average, sum_ceil.min(sum_floor));
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