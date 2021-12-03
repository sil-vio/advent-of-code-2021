use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;


fn compute_part_two(lines: &Vec<String>, column_size: usize) {
    let one_most_user_bit = is_one_most_used_bit(&lines, 0);
    let mut array_ox = filter_array(&lines, 0, if one_most_user_bit { '1' } else { '0' });
    let mut array_co2 = filter_array(&lines, 0, if one_most_user_bit { '0' } else { '1' });

    for i in 1..column_size {
        let one_most_user_bit = is_one_most_used_bit(&array_ox, i);
        let most_user_bit = if one_most_user_bit { ('1', '0') } else { ('0', '1') };
        array_ox = filter_array(&array_ox, i, most_user_bit.0);
        if array_ox.len() == 1 {
            break;
        }
    }

    for i in 1..column_size {
        let one_most_user_bit = is_one_most_used_bit(&array_co2, i);
        let most_user_bit = if one_most_user_bit { ('1', '0') } else { ('0', '1') };
        array_co2 = filter_array(&array_co2, i, most_user_bit.1);
        if array_co2.len() == 1 {
            break;
        }
    }
    let oxygen_generator_rating = i32::from_str_radix(&array_ox[0], 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(&array_co2[0], 2).unwrap();
    println!("oxygen_generator_rating {} , co2_scrubber_rating {},  What is the life support rating of the submarine? {}", oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);
}

fn compute_part_one(lines: &Vec<String>) {
    let column_size = lines.iter().next().unwrap().len();
    let mut gamma_rate_array: Vec<char> = Vec::new();
    let mut epsilon_rate_array: Vec<char> = Vec::new();
    for i in 0..column_size {
        let row_one_counter = is_one_most_used_bit(&lines, i);
        if row_one_counter {
            gamma_rate_array.push('1');
            epsilon_rate_array.push('0');
        } else {
            gamma_rate_array.push('0');
            epsilon_rate_array.push('1');
        }
    }
    let gamma_rate = i32::from_str_radix(&gamma_rate_array.iter().collect::<String>()[..], 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_array.iter().collect::<String>()[..], 2).unwrap();
    println!("gamma rate {} , epsilon rate {} = power consumption {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn is_one_most_used_bit(lines: &Vec<String>, position: usize) -> bool {
    let mut row_one_counter: u16 = 0;
    let mut size: u16 = 0;
    for i in 0..lines.len() {
        if lines[i].chars().collect::<Vec<char>>()[position] == '1' {
            row_one_counter += 1;
        }
        size += 1;
    }
    f32::from(row_one_counter) >= f32::from(size) / 2f32
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
    let column_size = lines.iter().next().unwrap().len();

    compute_part_one(&lines);
    compute_part_two(&lines, column_size);
}
