use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn compute_part_two(lines: &Vec<String>) {
    let column_size = lines.iter().next().unwrap().len();
    let ox_string_value = get_value_from_data(lines, column_size, true);
    let co2_string_value = get_value_from_data(lines, column_size, false);
    let oxygen_generator_rating = i32::from_str_radix(&ox_string_value, 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(&co2_string_value, 2).unwrap();
    println!("oxygen_generator_rating {} , co2_scrubber_rating {},  
        What is the life support rating of the submarine? {}", oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);
}

fn get_value_from_data(input_array: &Vec<String>, column_size: usize, is_ossigene: bool) -> String {
    let mut array = input_array.clone();
    for i in 0..column_size {
        let one_most_user_bit = is_one_most_used_bit_at_position(&array, i);
        let most_user_bit = if one_most_user_bit { ('1', '0') } else { ('0', '1') };
        array = filter_array(&array, i, if is_ossigene { most_user_bit.0 } else { most_user_bit.1 });
        if array.len() == 1 {
            break;
        }
    }
    array[0].to_string()
}

fn compute_part_one(lines: &Vec<String>) {
    let column_size = lines.iter().next().unwrap().len();
    let mut gamma_rate_array = String::new();
    let mut epsilon_rate_array = String::new();
    for i in 0..column_size {
        let one_most_used_bit = is_one_most_used_bit_at_position(&lines, i);
        if one_most_used_bit {
            gamma_rate_array.push('1');
            epsilon_rate_array.push('0');
        } else {
            gamma_rate_array.push('0');
            epsilon_rate_array.push('1');
        }
    }
    let gamma_rate = i32::from_str_radix(&gamma_rate_array, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_array, 2).unwrap();
    println!("gamma rate {} , epsilon rate {} = power consumption {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn is_one_most_used_bit_at_position(lines: &Vec<String>, position: usize) -> bool {
    let one_count: usize = lines.iter().filter(|entry| entry.chars().collect::<Vec<char>>()[position] == '1').count();
    one_count as f32>= lines.len() as f32/ 2f32
}

fn filter_array(input: &Vec<String>, position: usize, ch: char) -> Vec<String> {
    let mut result_array: Vec<String> = Vec::new();
    for i in 0..input.len() {
        if input[i].chars().collect::<Vec<char>>()[position] == ch {
            result_array.push(String::from(&input[i]));
        }
    }
    result_array
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

fn main() {
    let lines = lines_from_file("input").expect("Could not load lines");
    compute_part_one(&lines);
    compute_part_two(&lines);
}