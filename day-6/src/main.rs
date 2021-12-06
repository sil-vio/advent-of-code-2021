use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn calculate_fish(total_day: u16) -> u64 {
    let input = lines_from_file("input").unwrap();
    let mut map: HashMap<u16, u16> = HashMap::new();

    input.iter().for_each(|e| {
        let value = map.entry(*e).or_insert(0);
        *value += 1;
    });
    // let mut map_counter: HashMap<u16, Vec<u64>> = HashMap::new();
    let mut counter: Vec<u64> = vec![0,0,0,0,0,0,0,0,0];
    for i in 0..=8 {
        let entry= map.entry(i as u16).or_insert(0);
        counter[i] = *entry as u64;
    }
    // map_counter.entry(0).or_insert(counter.clone());
    // println!("day 0\t {:?} ", counter);

    for _day in 1..=total_day {
        let mut day_counter = vec![0,0,0,0,0,0,0,0,0];
        for position in 0..8 {
            day_counter[position] = counter[position + 1];
        }
        day_counter[6] += counter[0];
        day_counter[8] = counter[0];
        // println!("day {:?}\t {:?} ", day, day_counter);

        // map_counter.entry(day).or_insert(day_counter.clone());
        counter = day_counter.clone();
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
