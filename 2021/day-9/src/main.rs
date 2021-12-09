use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    value: u64,
}

fn main() {
    let input = lines_from_file("input").unwrap();
    let low_points = find_low_points(&input);
    println!("part 1 result: {:?} ", part1(&low_points));
    println!("part 2 result: {:?} ", part2(&input, &low_points));
}

fn part1(low_points: &Vec<Point>) -> u64 {
    let mut counter = 0;
    for point in low_points {
        counter += point.value + 1;
    }
    return counter;
}

fn find_low_points(input: &Vec<Vec<u64>>) -> Vec<Point> {
    let mut low_point: Vec<Point> = Vec::new();
    for y in 0..input.len() {
        let mut y_near = Vec::new();
        if y > 0 {
            y_near.push(y - 1);
        }
        if y < input.len() - 1 {
            y_near.push(y + 1);
        }
        y_near.push(y);
        for x in 0..input[y].len() {
            let mut x_near = Vec::new();
            if x > 0 {
                x_near.push(x - 1);
            }
            if x < input[y].len() - 1 {
                x_near.push(x + 1);
            }
            x_near.push(x);
            let mut is_min = true;
            for near_y in &y_near {
                for near_x in &x_near {
                    if ((*near_x != x || *near_y != y) && (x == *near_x || y == *near_y))
                        && input[y][x] >= input[*near_y][*near_x]
                    {
                        is_min = false;
                    }
                }
            }
            if is_min == true {
                println!(" found min in [{},{}] value {} ", y, x, input[y][x]);
                low_point.push(Point {
                    y,
                    x,
                    value: input[y][x],
                });
            }
        }
    }
    low_point
}

fn part2(input: &Vec<Vec<u64>>, low_points: &Vec<Point>) -> u64 {
    let mut basins = Vec::new();
    for low_point in low_points {
        let basin_point = calucalte_basin(input, &low_point.clone(), 0, 0, &HashSet::new());
        basins.push(basin_point.len() as u64);
    }
    basins.sort();
    basins.reverse();
    basins[0..3].iter().product()
}

fn calucalte_basin(
    input: &Vec<Vec<u64>>,
    point: &Point,
    direction_x: i32,
    direction_y: i32,
    basin_point: &HashSet<Point>,
) -> HashSet<Point> {
    let max_x = input[0].len();
    let max_y = input.len();
    let mut mybasin = basin_point.clone();
    mybasin.insert(point.clone());

    if point.x > 0 && (direction_x == 0 || direction_x == -1) && (input[point.y][point.x - 1] != 9) {

        let new_point = Point {
            y: point.y,
            x: point.x - 1,
            value: input[point.y][point.x - 1],
        };
        if basin_point.get(&new_point) == None {
            mybasin = calucalte_basin(input, &new_point, -1, 0, &mybasin);
        }
    }
    if point.x < max_x - 1 && (direction_x == 0 || direction_x == 1)  && (input[point.y][point.x + 1] != 9) {
        let new_point = Point {
            y: point.y,
            x: point.x + 1,
            value: input[point.y][point.x + 1],
        };
        if basin_point.get(&new_point) == None {
            mybasin = calucalte_basin(input, &new_point, 1, 0, &mybasin);
        }
    }
    if point.y > 0  && (direction_y == 0 || direction_y == -1) &&  (input[point.y - 1][point.x] != 9) {
        let new_point = Point {
            y: point.y - 1,
            x: point.x,
            value: input[point.y - 1][point.x],
        };
        if basin_point.get(&new_point) == None {
        
            mybasin = calucalte_basin(input, &new_point, 0, -1, &mybasin);
        }
    }
    if point.y < max_y - 1 && (direction_y == 0 || direction_y == 1) && (input[point.y + 1][point.x] != 9) {
        let new_point = Point {
            y: point.y + 1,
            x: point.x,
            value: input[point.y + 1][point.x],
        };
        if basin_point.get(&new_point) == None {
            mybasin = calucalte_basin(input, &new_point, 0, 1, &mybasin);
        }
    }
    return mybasin.clone();
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<u64>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_part1() {
        let lines = lines_from_file("test_input").unwrap();
        assert_eq!(lines.len(), 5);
        for line in &lines {
            assert_eq!(line.len(), 10);
        }
        assert_eq!(lines[2][2], 5);
        let low_points = find_low_points(&lines);
        assert_eq!(low_points.len(), 4);
        assert_eq!(part1(&low_points), 15);

        assert_eq!(part2(&lines, &low_points), 1134);
    }
}
