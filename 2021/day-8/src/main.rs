use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;


#[derive(Debug)]
struct Input {
    signal_pattern: Vec<String>,
    output_value: Vec<String>
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_singal, raw_output) = s.split_once('|').unwrap();
        Ok(Input{
            signal_pattern: raw_singal.trim_end().split(' ').map(|s| s.to_string()).collect(),
            output_value: raw_output.trim_start().split(' ').map(|s| s.to_string()).collect()
        })
    }
}

fn main() {
    let input = lines_from_file("input").unwrap();
    println!("input {:?} ", input);
    let counter = part1(input);
    println!("coutner {:?} ", counter);

    
}

fn part1(input: Vec<Input>) -> u16 {
    let mut counter = 0;
    for line in input {
        for output in line.output_value {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                counter += 1;
            }
        }
    }
    return counter;
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Input>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| Input::from_str(s.unwrap().as_str()).unwrap())
        .collect::<Vec<Input>>())
}


#[cfg(test)]
mod tests {
    use crate::{lines_from_file, part1};

    #[test]
    fn test_parsing_part1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 10); 
        println!("{:?} ", lines);
        for line in &lines {
            assert_eq!(line.signal_pattern.len(), 10);
            assert_eq!(line.output_value.len(), 4);
        }
        assert_eq!(part1(lines), 26);
    }

    
}