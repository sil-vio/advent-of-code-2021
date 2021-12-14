use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let rules = get_insertion_rules("input").unwrap();
    let template = get_polymer_template("input").unwrap();
    let part_one_timer = Instant::now();
    println!("diff = {} in ns {}", solution(template.clone(), rules.clone(), 10), part_one_timer.elapsed().as_nanos());
    let part_two_timer = Instant::now();
    println!("diff = {} in ns {}", solution(template.clone(), rules.clone(), 40), part_two_timer.elapsed().as_nanos());
}

fn solution(template: String, rules: HashMap<(char, char), char>, step: usize) -> usize {
    let couples: Vec<(char, char)> = get_substrings(&template);
    let mut freq_map: HashMap<(char,char), usize> = HashMap::new(); 
    for (a,b) in couples {
        *freq_map.entry((a,b)).or_insert(0) += 1;
    }
    for _ in 0..step {
        let mut new_freq_map = freq_map.clone();
        for ((a, b), f) in freq_map.iter() {
            if let Some(newchar) = rules.get(&(*a, *b)) {
                *new_freq_map.entry((*a, *newchar)).or_insert(0) += f;
                *new_freq_map.entry((*newchar, *b)).or_insert(0) += f;
                *new_freq_map.entry((*a, *b)).or_insert(0) -= f;
            }
        }
        freq_map = new_freq_map;
    }
    let mut freq_char : HashMap<char, (usize, usize)> = HashMap::new();
    for ((a, b), value) in freq_map {
        freq_char.entry(a).or_insert((0, 0)).0 += value;
        freq_char.entry(b).or_insert((0,0)).1 += value;
    }
    let mut max_freq_char:HashMap<char, usize> = HashMap::new();
    for (c, (l, r)) in freq_char {
        max_freq_char.entry(c).or_insert( l.max(r));
    }
    max_freq_char.iter().map(|(_,s)| s).max().unwrap() - max_freq_char.iter().map(|(_,s)| s).min().unwrap()
}


fn get_substrings(polymer: &String) -> Vec<(char, char)> {
    let mut substrings: Vec<(char, char)> = Vec::new();
    let mut polymer_chars = polymer.chars();
    let mut a = polymer_chars.nth(0).unwrap();
    loop {
        match polymer_chars.nth(0) {
            Some(b) => {
                substrings.push((a, b));
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

fn get_insertion_rules(filename: impl AsRef<Path>) -> io::Result<HashMap<(char, char), char>> {
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let file = File::open(filename)?;
    let raw_rule: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter(|l| l.as_ref().unwrap().contains(" -> "))
        .map(|l| l.unwrap())
        .collect();
    for rule in raw_rule {
        let (from, to) = rule.split_once(" -> ").unwrap();
        rules.insert(
            (from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()),
            to.chars().next().unwrap(),
        );
    }
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part2() {
        let rules = get_insertion_rules("test_input").unwrap();
        let template = get_polymer_template("test_input").unwrap();
        assert_eq!(solution(template, rules, 10), 1588);
    }
}
