use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn over_target(&self, x: i32, y: i32) -> bool {
        if x > self.x_max || y < self.y_min {
            true
        } else {
            false
        }
    }

    fn in_target(&self, x: i32, y: i32) -> bool {
        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            false
        } else {
            true
        }
    }
}

impl FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.replace("target area: ", "");
        println!("input to parse {}", input);
        let (x_input, y_input) = input.split_once(", ").unwrap();
        let x_input = x_input.replace("x=", "");
        let y_input = y_input.replace("y=", "");
        let (x_min, x_max) = x_input.split_once("..").unwrap();
        let (y_min, y_max) = y_input.split_once("..").unwrap();
        Ok(Target {
            x_min: i32::from_str(x_min).unwrap(),
            x_max: i32::from_str(x_max).unwrap(),
            y_min: i32::from_str(y_min).unwrap(),
            y_max: i32::from_str(y_max).unwrap(),
        })
    }
}

fn main() {
    let target = get_target_area("input").unwrap();
    let timer = Instant::now();
    let (hgt, hit_counter) = calculate_max_height_total_hit(&target);
    println!("height {} # hit {} -  in ns {}", hgt, hit_counter, timer.elapsed().as_nanos());
}

fn calculate_max_height_total_hit(target: &Target) -> (i32, usize) {
    let mut max_height = 0;
    let mut hit_counter = 0;
    let mut miss_counter = 0;
    let start_x = (2f32 * target.x_min as f32).sqrt() as i32;
    for vel_x in start_x..=target.x_max {
        for vel_y in target.y_min..=-target.y_min {
            let mut start_point = (0, 0);
            let (hit, t_max_h) = calculate_trajectory(&mut start_point, &mut (vel_x, vel_y), target);
            if hit {
                hit_counter += 1;
                max_height = max_height.max(t_max_h);
            } else {
                miss_counter += 1;
            }
        }
    }
    println!(" hit #: {}, miss #: {}", hit_counter, miss_counter);
    (max_height, hit_counter)
}

fn calculate_trajectory(point: &mut (i32, i32), velocity: &mut (i32, i32), target: &Target) -> (bool, i32) {
    let mut max_height = point.1;
    let mut in_target = false;
    loop {
        execute_step(point, velocity);
        max_height = max_height.max(point.1);
        if target.in_target(point.0, point.1) {
            in_target = true;
            break;
        }
        if target.over_target(point.0, point.1) {
            break;
        }
    }
    (in_target, max_height)
}

fn execute_step((x, y): &mut( i32, i32), (vx, vy): &mut( i32, i32)) {
    *x += *vx;
    *y += *vy;
    *vx = 0.max(*vx - 1);
    *vy -= 1;
}

fn get_target_area(filename: impl AsRef<Path>) -> io::Result<Target> {
    let file = File::open(filename)?;
    let lines: Vec<Target> = io::BufReader::new(file)
        .lines()
        .map(|line| Target::from_str(line.unwrap().as_str()).unwrap())
        .collect();
    Ok(lines[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let target = get_target_area("test_input").unwrap();
        assert_eq!(target.x_min, 20);
        assert_eq!(target.x_max, 30);
        assert_eq!(target.y_min, -10);
        assert_eq!(target.y_max, -5);
    }

    #[test]
    fn test_target_fn() {
        let target = get_target_area("test_input").unwrap();
        assert!(target.in_target(21, -6));
        assert!(!target.in_target(21, -2));
        assert!(!target.over_target(21, -2));
        assert!(!target.in_target(19, -6));
        assert!(!target.in_target(32, -6));
        assert!(target.over_target(32, -6));
        assert!(!target.in_target(22, -20));
        assert!(target.over_target(22, -20));
    }


    #[test]
    fn test_calculate_max_height_hit_counter() {
        let target = get_target_area("test_input").unwrap();
        let (max_height, hit_counter) = calculate_max_height_total_hit(&target);
        assert_eq!(max_height, 45);
        assert_eq!(hit_counter, 112);
    }
}
