use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Fold {
    horizzontal: bool,
    value: usize,
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_replaced = s.replace("fold along ", "");
        let (x, value) = s_replaced.split_once('=').unwrap_or(("a", "a"));
        Ok(Fold {
            horizzontal: x == "x",
            value: value.parse::<usize>()?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap_or(("a", "a"));
        Ok(Point {
            x: x.parse::<usize>()?,
            y: y.parse::<usize>()?,
        })
    }
}

fn main() {
    let points = get_points_from_file("input").unwrap();
    let folds = get_folds_from_file("input").unwrap();
    let now = Instant::now();
    solution_one(&points, &folds);
    println!("Day 13 A end in ns {}", now.elapsed().as_nanos());
    let now2 = Instant::now();
    solution_two(&points, &folds);
    println!("Day 13 B end in ns {}", now2.elapsed().as_nanos());

}

fn solution_one(points: &Vec<Point>, folds: &Vec<Fold>) {
    println!("start dots #: {}", points.len());

    let row_len = folds.iter().filter(|f| f.horizzontal).map(|p| p.value).max();
    let row_numbers = folds.iter().filter(|f| !f.horizzontal).map(|p| p.value).max();
    println!("row_len {}", row_len.unwrap() * 2 + 1);
    println!("row_numbers {}", row_numbers.unwrap() * 2 + 1);
    let mut rows = vec![vec![0; row_len.unwrap() * 2 + 1]; row_numbers.unwrap() * 2 + 1];
    for point in points {
        rows[point.y][point.x] = 1;
    }
    // print_rows(&rows);
    for fold in folds {
        println!("execute fold : {:?}", &fold);
        if fold.horizzontal {
            let mut new_rows = vec![vec![0; rows[0].len() / 2]; rows.len()];
            for y in 0..new_rows.len() {
                for x in 0..new_rows[y].len() {
                    new_rows[y][x] = if (rows[y][x] + rows[y][(rows[y].len() - 1) - x]) > 0 {
                        1
                    } else {
                        0
                    };
                }
            }
            rows = new_rows;
        } else {
            let mut new_rows = vec![vec![0; rows[0].len()]; rows.len() / 2];
            for y in 0..new_rows.len() {
                for x in 0..new_rows[y].len() {
                    new_rows[y][x] = if (rows[y][x] + rows[(rows.len() - 1) - y][x]) > 0 {
                        1
                    } else {
                        0
                    };
                }
            }
            rows = new_rows;
        }
        println!("dots #: {}", counts_dots(&rows));
    }
    print_rows(&rows);
}


fn solution_two(points: &Vec<Point>, folds: &Vec<Fold>) {
    println!("\n part 2 ");
    let mut set: HashSet<Point> = points.iter().map(|p| p.clone()).collect();
    println!("start dots #: {}", set.len());
    for fold in folds {
        println!("execute fold : {:?}", &fold);

        if fold.horizzontal {
            let new_set: HashSet<Point> = set.iter()
                .filter(|p| p.x > fold.value)
                .map(|p| Point{x: (fold.value*2 ) - p.x, y: p.y})
                .collect();
            set = set.union(&new_set).filter(|p| p.x < fold.value).map(|p| p.clone()).collect();
        } else {
            let new_set: HashSet<Point> = set.iter()
                .filter(|p| p.y > fold.value)
                .map(|p| Point{x: p.x, y: (fold.value *2) - p.y})
                .collect();
            set = set.union(&new_set).filter(|p| p.y < fold.value).map(|p| p.clone()).collect();

        }
        println!("dots #: {}", set.len());
    }
    let max_x = set.iter().map(|p| p.x).max().unwrap();
    let max_y = set.iter().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        let mut row = String::new();
        for x in 0..=max_x {
            let p = Point{x,y};
            match set.get(&p) {
                Some(_) => row.push('#'),
                None => row.push('.')
            }
        }
        println!("{}", row);
    }

    
}

fn print_rows(rows: &Vec<Vec<i32>>) {
    for row in rows {
        print!("\n");
        for cell in row {
            print!("{}", if *cell != 1i32 { '.' } else { '#' });
        }
    }
}

fn counts_dots(rows: &Vec<Vec<i32>>) -> usize {
    let mut counter = 0;
    for row in rows {
        for cell in row {
            if *cell > 0 {
                counter += 1;
            }
        }
    }
    counter
}

fn get_points_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Point>> {
    let file = File::open(filename)?;
    let points: Vec<Point> = io::BufReader::new(file)
        .lines()
        .map(|line| Point::from_str(line.unwrap().as_str()))
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .collect();
    Ok(points)
}

fn get_folds_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Fold>> {
    let file = File::open(filename)?;
    let folds: Vec<Fold> = io::BufReader::new(file)
        .lines()
        .map(|line| Fold::from_str(line.unwrap().as_str()))
        .filter(|l| l.is_ok())
        .map(|l| l.unwrap())
        .collect();
    Ok(folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let points = get_points_from_file("test_input").unwrap();
        let folds = get_folds_from_file("test_input").unwrap();
        assert_eq!(points.len(), 18);
        assert_eq!(folds.len(), 2);
        solution_one(&points, &folds);
        solution_two(&points, &folds);

    }
}
