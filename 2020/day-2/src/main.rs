use std::ops::RangeInclusive;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Entry {
    rule: PasswordRule,
    password: String,
}

#[derive(Debug, Clone)]
struct PasswordRule {
    range: RangeInclusive<u8>,
    chars_in: String,
}

fn main() {
    let lines = lines_from_file("input").unwrap();
    let count = check_password_validity_part1(lines.clone());
    println!("password valide {}", count);
    let count = check_password_validity_part2(lines);
    println!("password valide {}", count);


}

fn check_password_validity_part1(lines: Vec<Entry>) -> u16 {
    let mut counter = 0;

    for line in lines {
        let charto = line.rule.chars_in.chars().next().unwrap();

        let size = line.password.chars().filter(|e| *e == charto).count();
        println!("{:?} {:?} {:?}",line, charto, size);
        if line.rule.range.contains(&(size as u8)) {
            println!("valid! {:?}", line);
            counter += 1;
        }
    }

    counter
}

fn check_password_validity_part2(lines: Vec<Entry>) -> u16 {
    let mut counter = 0;

    for line in lines {
        let charto = line.rule.chars_in.chars().next().unwrap();

        let size = line.password.chars().filter(|e| *e == charto).count();
        if size >= 1 {
            if (line.password.chars().nth((*line.rule.range.start() - 1)  as usize).unwrap() == charto && line.password.chars().nth((*line.rule.range.end() - 1) as usize).unwrap() != charto) ||
               (line.password.chars().nth((*line.rule.range.start() - 1)  as usize).unwrap() != charto && line.password.chars().nth((*line.rule.range.end() - 1) as usize).unwrap() == charto) {
                counter += 1;
                println!(" line is valid {:?}", line);
            } 
        }
    }

    counter
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Entry>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| s.unwrap())
        .map(|s| {
            let (rule_raw, password) = s.split_once(": ").unwrap();
            let (range, char_in_password) = rule_raw.split_once(' ').unwrap();
            let (min, max) = range.split_once('-').unwrap();
            Ok(Entry {
                rule: PasswordRule {
                    range: RangeInclusive::new(min.parse().unwrap(), max.parse().unwrap()),
                    chars_in: char_in_password.to_string(),
                },
                password: password.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{check_password_validity_part1, check_password_validity_part2, lines_from_file};

    #[test]
    fn part_1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 3); 
        assert_eq!(check_password_validity_part1(lines), 2);
    }

     #[test]
    fn part_2() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 3); 
        assert_eq!(check_password_validity_part2(lines), 1);
    }
}
