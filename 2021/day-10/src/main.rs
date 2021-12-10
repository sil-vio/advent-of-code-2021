use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input").unwrap();
    let result_part_1 = part1(&lines);
    println!("part 1 : {}", result_part_1);
}

fn part1(lines: &Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let charmap: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut error_map: HashMap<char, u64> = HashMap::from([(')', 0), ('>', 0), ('}', 0), (']', 0)]);
    let point_map: HashMap<char, u64> = HashMap::from([(')', 3), ('>', 25137), ('}', 1197), (']', 57)]);

    for line in lines {
        let mut stack = Vec::new();
        // if line.len() % 2 == 0 {
            for c in line {
                match charmap.get(c) {
                    Some(_) => stack.push(c),
                    None => match stack.last() {
                        Some(last_char) => {
                            if charmap.get(&last_char).unwrap() == c {
                                stack.pop();
                            } else {
                                let value = error_map.entry(*c).or_insert(0);
                                *value += 1;
                                break;
                            }
                        }
                        None => {
                            let value = error_map.entry(*c).or_insert(0);
                            *value += 1;
                            break;
                        }
                    },
                }
            }
        //}
    }
    for entry in error_map {
        let coefficent = point_map.get(&entry.0).unwrap();
        result += entry.1 * coefficent;
    }

    result
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<char>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| s.unwrap().chars().map(|c| c).collect())
        .collect::<Vec<Vec<char>>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].len(), 24);
        let result = part1(&lines);
        assert_eq!(result, 26397);
    }
}
