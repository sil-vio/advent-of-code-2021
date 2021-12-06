use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn calculate_fish(total_day: u16) -> u64 {
    let mut counter = [0;9];
    lines_from_file("input").unwrap().iter().for_each(|e| {
        counter[*e as usize] += 1;
    });
    for _day in 1..=total_day {
        let zerovalue = counter[0];
        counter.rotate_left(1);
        counter[6] += zerovalue;
        counter[8] = zerovalue;
    }
    counter.iter().fold(0, |acc, x| acc+x)
}

fn main() {
    let fish_number_day_80 = calculate_fish(80);
    let fish_number_day_256 = calculate_fish(256);
    println!("Total Lanternfish after 80 days = {:?}", fish_number_day_80);
    println!("Total Lanternfish after 80 days = {:?}", fish_number_day_256);
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<u16>> {
    let line = BufReader::new(File::open(filename)?)
        .lines()
        .next()
        .unwrap()
        .unwrap();
    Ok(line
        .split(',')
        .map(|e| e.parse::<u16>().unwrap())
        .collect::<Vec<u16>>())
}
