use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("input").unwrap();
    let result = part1(&lines);
    println!("part 1 : {}", result.0);
    println!("part 2 : {}", result.1);
}

fn part1(lines: &Vec<Vec<char>>) -> (u64, u64) {
    let mut result_1 = 0;
    let charmap: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut error_map: HashMap<char, u64> = HashMap::from([(')', 0), ('>', 0), ('}', 0), (']', 0)]);
    let part1_point_map: HashMap<char, u64> =
        HashMap::from([(')', 3), ('>', 25137), ('}', 1197), (']', 57)]);

    let mut array_score: Vec<u64> = Vec::new();

    for line in lines {
        let mut stack = Vec::new();
        let mut corrupted = false;
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
                            corrupted = true;
                            break;
                        }
                    }
                    None => {
                        let value = error_map.entry(*c).or_insert(0);
                        *value += 1;
                        corrupted = true;
                        break;
                    }
                },
            }
        }
        if !corrupted && stack.len() > 0 {
            let mut closing_sequence = Vec::new();
            stack.reverse();
            for c in stack {
                match charmap.get(c) {
                    Some(inverse_c) => closing_sequence.push(inverse_c),
                    None => panic!("errore carattere non trovato {}", c),
                }
            }
            array_score.push(closing_sequence_score(closing_sequence));
        }
    }
    for entry in error_map {
        let coefficent = part1_point_map.get(&entry.0).unwrap();
        result_1 += entry.1 * coefficent;
    }
    array_score.sort();
    (result_1, array_score[(array_score.len() / 2 ) as usize])
}

fn closing_sequence_score(closing_sequence: Vec<&char>) -> u64 {
    let mut result = 0;
    let part2_point_map: HashMap<char, u64> =
        HashMap::from([(')', 1), ('>', 4), ('}', 3), (']', 2)]);
    for c in closing_sequence {
        result = result * 5;
        result += part2_point_map.get(c).unwrap();
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
    fn test_day_9() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].len(), 24);
        let result = part1(&lines);
        assert_eq!(result.0, 26397);
        assert_eq!(result.1, 288957);

    }


    #[test]
    fn test_closing_sequence_score() {
        let input_1 = vec![&'}', &'}', &']', &']', &')', &'}', &')',&']'];
        assert_eq!(closing_sequence_score(input_1), 288957);
    }
}
