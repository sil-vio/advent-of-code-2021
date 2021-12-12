use std::collections::HashMap;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use regex::Regex;

fn main() {
    let entries_part1 = entries_with_required_fields("input").unwrap();
    println!("entry valid part 1 : {}", entries_part1.len());
    let valid_entries_counter = entries_valid(&entries_part1);
    println!("entry valid part 2 : {}", valid_entries_counter);
}
fn entries_valid(entries_part1: &Vec<HashMap<String, String>>) -> u64 {
    let mut valid_counter = 0;
    // hcl  a # followed by exactly six characters 0-9 or a-f.
    let re_hcl = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();

    let valid_eye_color = ["amb","blu","brn","gry","grn","hzl","oth"];
    for entry in entries_part1 {
        let byr = entry.get("byr").unwrap().parse::<u64>().unwrap_or(1900);
        if byr < 1920 || byr > 2002 {
            println!("not valid byr {}", byr);
            continue;
        }
        let iyr = entry.get("iyr").unwrap().parse::<u64>().unwrap_or(1900);
        if iyr < 2010 || iyr > 2020 {
            println!("not valid iyr {}", iyr);
            continue;
        }
        let eyr = entry.get("eyr").unwrap().parse::<u64>().unwrap_or(1900);
        if eyr < 2020 || eyr > 2030 {
            println!("not valid eyr {}", eyr);
            continue;
        }
        let hgt = entry.get("hgt").unwrap();
        if hgt.ends_with("cm") || hgt.ends_with("in") {
            if hgt.ends_with("cm") {
                let (l, r) = hgt.split_once("cm").unwrap();
                let heigth = l.parse::<u64>().unwrap_or(0);
                if heigth < 150 || heigth > 193 {
                    println!("not valid heigth {}", heigth);
                    continue;
                }
            } else {
                let (l, r) = hgt.split_once("in").unwrap();
                let heigth = l.parse::<u64>().unwrap_or(0);
                if heigth < 59 || heigth > 76 {
                    println!("not valid heigth {}", heigth);
                    continue;
                }
            }
        } else {
            continue;
        }
        let hcl = entry.get("hcl").unwrap();
        if !re_hcl.is_match(hcl) {
            println!("not valid hcl {}", hcl);
            continue;
        }
        let ecl = entry.get("ecl").unwrap();
        if !valid_eye_color.contains(&ecl.as_str()) {
            println!("not valid ecl {}", ecl);
            continue;
        } 
        let pid = entry.get("pid").unwrap();
        if !re_pid.is_match(pid) {
            println!("not valid pid {}", pid);
            continue;
        }
        valid_counter += 1;
    }
    valid_counter
}

fn entries_with_required_fields(
    filename: impl AsRef<Path>,
) -> io::Result<Vec<HashMap<String, String>>> {
    let lines = BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| s.unwrap())
        //.map(|s| s.replace("\n", "|"))
        .flat_map(|s| {
            if s.is_empty() {
                vec![s; 1]
            } else {
                s.split_whitespace()
                    .map(|sub| sub.to_string())
                    .collect::<Vec<String>>()
            }
        })
        .collect::<Vec<String>>();
    let mut result: Vec<HashMap<String, String>> = Vec::new();
    let mut entry = HashMap::new();
    for line in &lines {
        if line.len() > 0 {
            let (key, value) = line.split_once(":").unwrap();
            entry.insert(key.to_string(), value.to_string());
        } else {
            if entry.len() == 8 || (entry.len() == 7 && entry.get("cid") == None) {
                result.push(entry);
            }
            entry = HashMap::new();
        }
    }
    if entry.len() == 8 || (entry.len() == 7 && entry.get("cid") == None) {
        result.push(entry);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexp() {
        let re_hcl = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
        assert_eq!(!re_hcl.is_match("#698d72"), false);

        let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();
        assert_eq!(!re_pid.is_match("001642707"), false);

    }

    #[test]
    fn day_3() {
        let lines = entries_with_required_fields("test_input").unwrap();
        assert_eq!(lines.len(), 2);
        let part2 = entries_valid(&lines);
        assert_eq!(part2, 2);

    }
}
