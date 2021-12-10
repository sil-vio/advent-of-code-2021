use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
 
fn main() {
    let lines = lines_from_file("input").unwrap();
    let mut values =vec![0; 5];
    values[0] = check_slope(&lines, 1, 1);
    values[1] = check_slope(&lines, 3, 1);
    values[2] = check_slope(&lines, 5, 1);
    values[3] = check_slope(&lines, 7, 1);
    values[4] = check_slope(&lines, 1, 2);
    println!("part 1 : {}", values[1]);
    println!("part 2 : {}", values.iter().product::<u64>());
}

fn check_slope(lines: &Vec<Vec<char>>, dx_slope: usize, down_slope: usize) -> u64 {
    let mut counter: u64 = 0;
    for (i, line) in lines.iter().step_by(down_slope).enumerate() {
        if line[i*dx_slope % lines[0].len()] == '#' {
            counter += 1;
        }
    } 
    counter
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
    fn day_3() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 11); 
        assert_eq!(lines.get(0).unwrap().len(), 11);
        assert_eq!(check_slope(&lines, 1, 1), 2);
        assert_eq!(check_slope(&lines, 3, 1), 7);
        assert_eq!(check_slope(&lines, 5, 1), 3);
        assert_eq!(check_slope(&lines, 7, 1), 4);
        assert_eq!(check_slope(&lines, 1, 2), 2);
    }

}
