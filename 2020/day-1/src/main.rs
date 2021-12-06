use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("input").expect("Could not load lines");

    let set: HashSet<u16> = HashSet::from_iter(lines);
    println!("part 1 {}", part1(set.clone()).unwrap());
    println!("part 2 {}", part2(set.clone()).unwrap());
}

fn part1(set: HashSet<u16>) -> Option<u32> {
    seek_value(set, 2020u16)
}

fn part2(set: HashSet<u16>) -> Option<u32> {
    for ele in set.clone().iter() {
        if let Some(value) = seek_value(set.clone(), 2020 - ele) {
            return Some(*ele as u32 * value);
        }
    }
    None
}

fn seek_value(set: HashSet<u16>, seek: u16) -> Option<u32> {
    for ele in set.iter() {
        if *ele > seek {
            continue;
        }
        let opposite = seek - *ele;
        if let Some(_) = set.get(&opposite) {
            let result = *ele as u32 * opposite as u32;
            return Some(result);
        }
    }
    None
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<u16>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| x.map(|t| t.parse::<u16>().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{lines_from_file, part1, part2};

    #[test]
    fn part_1() {
        let lines = lines_from_file("test_input").expect("Could not load lines");
        let set: HashSet<u16> = HashSet::from_iter(lines);
        
        assert_eq!(part1(set).unwrap(), 514579);
    }

    #[test]
    fn part_2() {
        let lines = lines_from_file("test_input").expect("Could not load lines");
        let set: HashSet<u16> = HashSet::from_iter(lines);
    
        assert_eq!(part2(set).unwrap(), 241861950);

    }
}
