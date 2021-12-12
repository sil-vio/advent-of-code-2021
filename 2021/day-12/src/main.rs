use std::{collections::{HashSet}, fs::File, io::{self, BufRead}, path::Path, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cave {
    name: String,
    small: bool,
    start: bool,
    end: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Line {
    a: Cave,
    b: Cave,
    start: bool,
    end: bool,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once("-")
            .map(|(x, y)| (Cave::from_str(x).unwrap(), Cave::from_str(y).unwrap()))
            .unwrap();
        Ok(Line {
            a: a.clone(),
            b: b.clone(),
            start: a.start,
            end: b.end
        })
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cave { name: s.to_string(), small: !(s.to_uppercase() == s) && s != "end", end: s == "end", start: s == "start"})
    }
}

fn read_lines<P>(filename: P) -> io::Result<Vec<Line>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut lines: Vec<Line> = Vec::new();
    let mut inverted_line: Vec<Line> = Vec::new();
    io::BufReader::new(file).lines().for_each(|line| lines.push(Line::from_str(line.unwrap().as_str()).unwrap()));
    lines.iter().for_each(|line| {
        if line.end == false && line.start == false{ // && (!line.a.small || !line.b.small)  {
            inverted_line.push(Line{
                a: Cave{
                    name: line.b.name.clone(),
                    small: line.b.small,
                    start: line.b.start,
                    end: line.b.end
                },
                b: Cave{
                    name: line.a.name.clone(),
                    small: line.a.small,
                    start: line.a.start,
                    end: line.a.end 
                },
                start: false,
                end: false
            })
        }
    }) ;
    inverted_line.iter().for_each(|line| lines.push(line.clone()));
    Ok(lines)
}

fn main() {
    let input = read_lines("input").unwrap();
    let counter_part_1 = generate_path(input.clone(), false);
    println!("Part 1 #: {}", counter_part_1);
    let counter_part_1 = generate_path(input.clone(), true);
    println!("Part 2 #: {}", counter_part_1);
}   

fn generate_path(input: Vec<Line>, allow_one_duplicate: bool) -> u64 {
    let mut counter = 0;
    let mut i = 0;
    let mut lista_parole: Vec<Vec<Line>> = Vec::new();
    lista_parole.push(vec![Line{a: Cave{name: "-->".to_string(), small: false, start: false, end: false }, b: Cave{name: "start".to_string(), small: false, start: true, end: false}, start: true, end: false}]);
    loop {
        if !lista_parole[i].last().unwrap().end {
            let next_steps: Vec<Line> = get_next_steps(&input,  &lista_parole[i], allow_one_duplicate);
             for next_step in next_steps {
                let mut new_line = lista_parole[i].clone();
                new_line.push(next_step.clone());
                lista_parole.push(new_line.clone());
            }
        } else {
            //let final_path = lista_parole[i].iter().map(|e| e.b.name.to_string()).collect::<Vec<String>>().join(",");
            //println!("{}", final_path);
            counter +=1;
        }
        i += 1;
        if lista_parole.len() == i {
            break;
        }
    }
    counter
}

fn get_next_steps(input: &Vec<Line>, partial_list: &Vec<Line>, allow_one_duplicate: bool) -> Vec<Line> {
    let mut cave_set: HashSet<Cave> = HashSet::new(); 
    partial_list.iter().for_each(|entry | {
        cave_set.insert(entry.a.clone());
        cave_set.insert(entry.b.clone());
    });
    let mut duplicate = false;
    let small_caves_in_path = partial_list
        .iter()
        .map(|e| e.b.clone())
        .filter(|e| e.small)
        .collect::<Vec<Cave>>();


    if (1..small_caves_in_path.len())
        .any(|i| small_caves_in_path[i..].contains(&small_caves_in_path[i - 1]))
    {
        // println!("found duplicate in {:?} ", small_caves_in_path);
        duplicate = true;
    }

    if allow_one_duplicate {
         return input
             .iter()
             .filter(|line| line.a.name == partial_list.last().unwrap().b.name.to_string())
             .filter(|line| !line.b.small || !duplicate || !cave_set.contains(&line.b))
             .map(|line| line.to_owned())
             .collect();
    }
    return input.iter()
        .filter(|line | line.a.name == partial_list.last().unwrap().b.name.to_string())
        .filter(|line| !line.b.small || (line.b.small && !cave_set.contains(&line.b)))
        .map(|line | line.to_owned()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1a() {
        let lines = read_lines("test_input").unwrap();
        // println!("lines {:#?}", lines);
        assert_eq!(generate_path(lines.clone(), false), 10);
    }

    #[test]
    fn test_parsing_part2a() {
        let lines = read_lines("test_input").unwrap();
        // println!("lines {:#?}", lines);
        assert_eq!(generate_path(lines.clone(), true), 36);
    }
    

    #[test]
    fn test_parsing_part1b() {
        let lines = read_lines("test_input_2").unwrap();
        assert_eq!(generate_path(lines, false), 19);
      
    }


    #[test]
    fn test_parsing_part2b() {
        let lines = read_lines("test_input_2").unwrap();
        assert_eq!(generate_path(lines, true), 103);
      
    }

    #[test]
    fn test_parsing_part1c() {
        let lines = read_lines("test_input_3").unwrap();
        assert_eq!(generate_path(lines, false), 226);
      
    }

    #[test]
    fn test_parsing_part2c() {
        let lines = read_lines("test_input_3").unwrap();
        assert_eq!(generate_path(lines, true), 3509);
      
    }
}
