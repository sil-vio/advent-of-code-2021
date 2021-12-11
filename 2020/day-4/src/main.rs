use std::collections::HashMap;
use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
 

fn main() {
    let entries_part1 = entries_with_required_fields("input").unwrap();
    println!("entry valid part 1 : {}", entries_part1.len());
    let valid_entries_counter = entries_valid(&entries_part1);
    println!("entry valid part 2 : {}", valid_entries_counter);


}

fn entries_valid(entries_part1: &Vec<HashMap<String, String>>) -> u64 {
    0u64
}


fn entries_with_required_fields(filename: impl AsRef<Path>) -> io::Result<Vec<HashMap<String, String>>> {
    let lines = BufReader::new(File::open(filename)?).lines().map(|s| s.unwrap())
        //.map(|s| s.replace("\n", "|"))
        .flat_map(|s| if s.is_empty() { vec![s; 1] } else { s.split_whitespace().map(|sub| sub.to_string()).collect::<Vec<String>>() })
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
    fn day_3() {
        let lines = entries_with_required_fields("test_input").unwrap();
        assert_eq!(lines.len(), 2); 
    }

}
