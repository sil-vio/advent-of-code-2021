use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input").expect("Could not load lines");

    println!("input size {}", lines.len());
    let counter = lines.windows(2).filter(|vec| vec[1] > vec[0]).count();
    let counter2 = lines.windows(4).filter(|vec| vec[3] > vec[0]).count();

    println!("counter 1: {}", counter);
    println!("counter 2: {}", counter2);
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<u16>> {
    BufReader::new(File::open(filename)?).lines().map(|x| x.map(|t| {t.parse::<u16>().unwrap()})).collect()
}
