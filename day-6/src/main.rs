use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut input = lines_from_file("input").unwrap();
    for day in 0..256 {
        println!("start day {:?}", day);
        let mut new_gen_counter = 0;
        for value in &mut input {
            if *value == 0u16 {
                new_gen_counter += 1;
                *value = 6u16;
            } else {
                *value -=1u16;
            }         
        }
        // println!("day {:?} add {}", day, new_gen_counter);
        for _ in 0..new_gen_counter {
            input.push(8u16);
        }
    }
    println!("total fish: {}", input.len());
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<u16>> {
    let line = BufReader::new(File::open(filename)?).lines()
    .next().unwrap()
    .unwrap();
    Ok(line.split(',').map(|e| e.parse::<u16>().unwrap()).collect::<Vec<u16>>())

}