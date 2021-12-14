use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let rules = get_insertion_rules("input").unwrap();
    let template = get_polymer_template("input").unwrap(); 
    println!("diff = {}", part1(template.clone(), rules.clone(), 10));
    println!("diff = {}", part1(template.clone(), rules.clone(), 40));
}

fn part1(template: String, rules: HashMap<String, String>, step: usize) -> usize {
    let mut polymer = String::from(template);
    for i in 0..step {
        println!("execute step {}", i);
        let substrings: Vec<String> = get_substrings(&polymer);
        let last_char = polymer.chars().nth(polymer.len() - 1).unwrap();
        polymer.clear();
        for substring in substrings {
            let to = rules.get(&substring).unwrap();
            polymer.push(substring.chars().nth(0).unwrap());
            polymer.push(to.chars().nth(0).unwrap());
        }
        polymer.push(last_char);
    }
    let charset = polymer.chars().collect::<HashSet<char>>();
    let mut sizevec: Vec<usize> = Vec::new();
    for filtro in charset.iter() {
        let count = polymer.chars().filter(|c| c == filtro).count();
        println!("char {} = {}", filtro, count);
        sizevec.push(count);
    }
    sizevec.iter().max().unwrap() - sizevec.iter().min().unwrap()
}

fn get_substrings(polymer: &String) -> Vec<String> {
    let mut substrings: Vec<String> = Vec::new();
    let mut polymer_chars = polymer.chars();
    let mut a = polymer_chars.nth(0).unwrap();
    loop {
        let mut substring = String::new();
        match polymer_chars.nth(0) {
            Some(b) => {
                substring.push(a);
                substring.push(b);
                substrings.push(substring);
                a = b.clone();
            }
            None => break,
        }
    }
    substrings
}

fn get_polymer_template(filename: impl AsRef<Path>) -> io::Result<String> {
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(lines[0].clone())
}

fn get_insertion_rules(filename: impl AsRef<Path>) -> io::Result<HashMap<String, String>> {
    let mut rules: HashMap<String, String> = HashMap::new();
    let file = File::open(filename)?;
    let raw_rule: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter(|l| l.as_ref().unwrap().contains(" -> "))
        .map(|l| l.unwrap())
        .collect();
    for rule in raw_rule {
        let (from, to) = rule.split_once(" -> ").unwrap();
        rules.insert(from.to_string(), to.to_string());
    }
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let rules = get_insertion_rules("test_input").unwrap();
        let template = get_polymer_template("test_input").unwrap();
        println!("{:#?}", rules);
        assert_eq!(template, "NNCB");
        let substrings = get_substrings(&template);
        assert_eq!(substrings.len(), 3);
        assert_eq!(rules.len(), 16);
        assert_eq!(part1(template, rules, 10), 1588);
    }
}
