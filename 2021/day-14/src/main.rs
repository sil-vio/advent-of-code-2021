use std::array::from_ref;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Rule {
    from: String,
    to: String,
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").unwrap();
        Ok(Rule {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

fn main() {
    
}

fn part1(template: String, rules: Vec<Rule>) -> usize {

    0
}

fn get_polymer_template(filename: impl AsRef<Path>) -> io::Result<String> {
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(lines[0].clone())
}

fn get_insertion_rules(filename: impl AsRef<Path>) -> io::Result<Vec<Rule>> {
    let file = File::open(filename)?;
    let folds: Vec<Rule> = io::BufReader::new(file)
        .lines()
        .filter(|l| l.as_ref().unwrap().contains(" -> "))
        .map(|line| Rule::from_str(line.unwrap().as_str()))
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .collect();
    Ok(folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let rules = get_insertion_rules("test_input").unwrap();
        let template = get_polymer_template("test_input").unwrap();
        assert_eq!(template, "NNCB");
        assert_eq!(rules.len(), 16);

    }
}
