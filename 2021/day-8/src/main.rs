use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    signal_pattern: Vec<String>,
    output_value: Vec<String>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_singal, raw_output) = s.split_once('|').unwrap();
        Ok(Input {
            signal_pattern: raw_singal
                .trim_end()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
            output_value: raw_output
                .trim_start()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        })
    }
}

fn main() {
    let input = lines_from_file("input").unwrap();
    println!("part 1 result: {:?} ", part1(&input));
    println!("part 2 result: {:?} ", part2(&input));
}

fn part1(input: &Vec<Input>) -> u16 {
    let mut counter = 0;
    for line in input {
        for output in &line.output_value {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                counter += 1;
            }
        }
    }
    return counter;
}

fn part2(input: &Vec<Input>) -> u64 {
    let mut output_sum = 0;
    for line in input {
        let decoded_output = decode_output(line);
        output_sum += decoded_output;
    }
    output_sum
}

fn decode_output(line: &Input) -> u64 {
    let mut dictionary_one: HashMap<String, u64> = HashMap::new();
    let mut dictionary_two: HashMap<u64, String> = HashMap::new();

    for signal in &line.signal_pattern {
        if signal.len() == 2 {
            dictionary_one.insert(signal.to_string(), 1);
            dictionary_two.insert(1, signal.to_string());
        } else if signal.len() == 4 {
            dictionary_one.insert(signal.to_string(), 4);
            dictionary_two.insert(4, signal.to_string());
        } else if signal.len() == 3 {
            dictionary_one.insert(signal.to_string(), 7);
            dictionary_two.insert(7, signal.to_string());
        } else if signal.len() == 7 {
            dictionary_one.insert(signal.to_string(), 8);
            dictionary_two.insert(8, signal.to_string());
        }
    }
    // println!("r1 dictionary_one: {:?} ", dictionary_one);
    for signal in &line.signal_pattern {
        if is_number_three(signal, dictionary_two.get(&1).unwrap()) {
            dictionary_one.insert(signal.to_string(), 3);
            dictionary_two.insert(3, signal.to_string());
        } else if is_number_nine(
            signal,
            dictionary_two.get(&4).unwrap(),
            dictionary_two.get(&1).unwrap(),
        ) {
            dictionary_one.insert(signal.to_string(), 9);
            dictionary_two.insert(9, signal.to_string());
        } else if is_number_zero(
            signal,
            dictionary_two.get(&1).unwrap(),
            dictionary_two.get(&4).unwrap(),
        ) {
            dictionary_one.insert(signal.to_string(), 0);
            dictionary_two.insert(0, signal.to_string());
        } if is_number_five(signal, dictionary_two.get(&4).unwrap(), dictionary_two.get(&1).unwrap()) {
            dictionary_one.insert(signal.to_string(), 5);
            dictionary_two.insert(5, signal.to_string());
        }
    }
    for signal in &line.signal_pattern  {
        if is_number_two(signal, dictionary_two.get(&3).unwrap(), dictionary_two.get(&5).unwrap()) {
            dictionary_one.insert(signal.to_string(), 2);
            dictionary_two.insert(2, signal.to_string());
        } else if is_number_six(signal, dictionary_two.get(&9).unwrap(), dictionary_two.get(&0).unwrap()) {
            dictionary_one.insert(signal.to_string(), 6);
            dictionary_two.insert(6, signal.to_string());
        }   
    }

    let mut result:u64 = 0;
    for i in 0..4 {
        let value = dictionary_one.keys().filter(|key | key.len() == line.output_value[i].len() && contains_all_char(*key, &line.output_value[i]) == true ).collect::<Vec<&String>>();
        let mut entry_value = *dictionary_one.get(value[0]).unwrap();
        if i == 0 {
            entry_value = entry_value * 1000 as u64;
        } else if i == 1  {
            entry_value = entry_value * 100 as u64;
        } else if i == 2 {
            entry_value = entry_value * 10 as u64;
        }
        result += entry_value;
    }
    result
}

fn is_number_zero(input: &String, number_one: &String, number_four: &String) -> bool {
    if input.len() == 6 {
        return contains_all_char(input, number_one) && not_contains_all_char(input, number_four);
    }
    false
}

fn is_number_two(input: &String, number_three: &String, number_five: &String) -> bool {
    if input.len() == 5 {
        return not_contains_all_char(input, number_three) && not_contains_all_char(input, number_five);
    }
    false
}

fn is_number_three(input: &String, number_one: &String) -> bool {
    if input.len() == 5 {
        return contains_all_char(input, number_one);
    }
    false
}

fn is_number_five(input: &String, number_four: &String, number_one: &String) -> bool {
    if input.len() == 5 {
        return number_char_in_common(input, number_four, 3) && not_contains_all_char(input, number_one);
    }
    false
}

fn is_number_six(input: &String, number_nine: &String, number_zero: &String) -> bool {
    if input.len() == 6 {
        return not_contains_all_char(input, number_nine) && not_contains_all_char(input, number_zero);
    }
    false
}

fn is_number_nine(input: &String, number_four: &String, number_one: &String) -> bool {
    if input.len() == 6 {
        return contains_all_char(input, number_one) && contains_all_char(input, number_four);
    }
    false
}

fn contains_all_char(input: &String, chars_to_contains: &String) -> bool {
    return number_char_in_common(input, chars_to_contains, chars_to_contains.len());
}

fn not_contains_all_char(input: &String, chars_to_not_contains: &String) -> bool {
    return chars_to_not_contains
        .chars()
        .filter(|cf| input.chars().filter(|c| c == cf).count() == 1)
        .count()  != chars_to_not_contains.len();
}

fn number_char_in_common(
    input: &String,
    chars_to_contains: &String,
    num_char_in_common: usize,
) -> bool {
    return chars_to_contains
        .chars()
        .filter(|cf| input.chars().filter(|c| c == cf).count() == 1)
        .count()
        == num_char_in_common;
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Input>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| Input::from_str(s.unwrap().as_str()).unwrap())
        .collect::<Vec<Input>>())
}

#[cfg(test)]
mod tests {
    use crate::{contains_all_char, is_number_five, is_number_nine, is_number_three, is_number_zero, lines_from_file, not_contains_all_char, part1, part2};

    #[test]
    fn test_not_contains_char() {
        assert!(not_contains_all_char(&"cagedb".to_string(), &"eafb".to_string()));
        assert!(not_contains_all_char(&"afc".to_string(), &"dfg".to_string()));
        assert!(!not_contains_all_char(&"cagedb".to_string(), &"age".to_string()));

    }

     #[test]
    fn test_is_number_zero() {
        assert!(is_number_zero(&"cagedb".to_string(), &"ab".to_string(),  &"eafb".to_string()));
        assert!(!is_number_zero(&"cdfgeb".to_string(), &"ab".to_string(),  &"eafb".to_string()));
    }

    #[test]
    fn test_is_number_three() {
        assert!(is_number_three(&"cdbaf".to_string(), &"ab".to_string()));
        assert!(!is_number_three(&"cdfeb".to_string(), &"ab".to_string()));
    }
    #[test]
    fn test_is_number_five() {
        assert!(is_number_five(&"cdfbe".to_string(), &"eafb".to_string(), &"ab".to_string()));
        assert!(!is_number_five(&"fbcad".to_string(), &"eafb".to_string(), &"ab".to_string()));
    }

    #[test]
    fn test_is_number_nine() {
        let number = &"cefabd".to_string();
        let number_one = &"ab".to_string();
        let number_four = &"eafb".to_string();
        assert!(contains_all_char(number, number_one));
        assert!(contains_all_char(number, number_four));
        assert!(is_number_nine(number, number_one, number_four));
    }

    #[test]
    fn test_parsing_part1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 10);
        for line in &lines {
            assert_eq!(line.signal_pattern.len(), 10);
            assert_eq!(line.output_value.len(), 4);
        }
        assert_eq!(part1(&lines), 26);
    }


    #[test]
    fn test_parsing_part2() {
        println!(r#"acedgfb: 8
cdfbe: 5
gcdfa: 2
fbcad: 3
dab: 7
cefabd: 9
cdfgeb: 6
eafb: 4
cagedb: 0
ab: 1"#);
        let lines = lines_from_file("test_input_2").unwrap();
        assert_eq!(part2(&lines), 5353);
    }


    #[test]
    fn test_parsing_part2b() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(part2(&lines), 61229);
    }
}
