use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Target {
    fn over_target(&self, x: i64, y: i64) -> bool {
        if x > self.x_max || y < self.y_min {
            true
        } else {
            false
        }
    }

    fn in_target(&self, x: i64, y: i64) -> bool {
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
            x_min: i64::from_str(x_min).unwrap(),
            x_max: i64::from_str(x_max).unwrap(),
            y_min: i64::from_str(y_min).unwrap(),
            y_max: i64::from_str(y_max).unwrap(),
        })
    }
}

fn main() {
    let target = get_target_area("input").unwrap();
    let part_one_timer = Instant::now();
    let (max_height, counter) = calculate_max_height_total_hit(&target);
    println!(
        "max_height {} total hit {} -  in ns {}",
        max_height,
        counter,
        part_one_timer.elapsed().as_nanos()
    );
    // let part_two_timer = Instant::now();
}

fn calculate_max_height_total_hit(target: &Target) -> (i64, usize) {
    let start_point = (0, 0);
    let mut max_height = 0;
    let mut hit_counter = 0;
    let mut miss_counter = 0;
    for vel_x in 3..=target.x_max {
        for vel_y in target.y_min..=-target.y_min {
            let (hit, trajectory_max_height) =
            calculate_trajectory(start_point, (vel_x, vel_y), target);
            if hit {
                hit_counter += 1;
                max_height = max_height.max(trajectory_max_height);
            } else {
                miss_counter += 1;
            }
        }
    }
    println!(" hit #: {}, miss #: {}", hit_counter, miss_counter);
    (max_height, hit_counter)
}

fn calculate_trajectory(point: (i64, i64), velocity: (i64, i64), target: &Target) -> (bool, i64) {
    let mut max_height = point.1;
    let mut in_target = false;
    let mut p = point.clone();
    let mut v = velocity.clone();
    loop {
        let (new_p, new_v) = execute_step(p, v);
        p = new_p.clone();
        v = new_v.clone();
        max_height = max_height.max(p.1);
        if target.in_target(p.0, p.1) {
            in_target = true;
            break;
        }
        if target.over_target(p.0, p.1) {
            break;
        }
    }
    (in_target, max_height)
}

fn execute_step(start_point: (i64, i64), velocity: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let new_point = (start_point.0 + velocity.0, start_point.1 + velocity.1);
    let new_velocity = (0.max(velocity.0 - 1), velocity.1 - 1);
    (new_point, new_velocity)
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
    fn test_execute_step() {
        let point = (0, 0);
        let velocity = (7, 2);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(point.0, 7);
        assert_eq!(point.1, 2);
        assert_eq!(velocity.0, 6);
        assert_eq!(velocity.1, 1);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(point.0, 13);
        assert_eq!(point.1, 3);
        assert_eq!(velocity.0, 5);
        assert_eq!(velocity.1, 0);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(point.0, 18);
        assert_eq!(point.1, 3);
        assert_eq!(velocity.0, 4);
        assert_eq!(velocity.1, -1);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(point.0, 22);
        assert_eq!(point.1, 2);
        assert_eq!(velocity.0, 3);
        assert_eq!(velocity.1, -2);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(velocity.0, 2);
        assert_eq!(velocity.1, -3);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(velocity.0, 1);
        assert_eq!(velocity.1, -4);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(velocity.0, 0);
        assert_eq!(velocity.1, -5);
        let (point, velocity) = execute_step(point, velocity);
        assert_eq!(velocity.0, 0);
        assert_eq!(velocity.1, -6);
        let (_, velocity) = execute_step(point, velocity);
        assert_eq!(velocity.0, 0);
        assert_eq!(velocity.1, -7);
    }

    #[test]
    fn test_calculate_trajectory() {
        let target = get_target_area("test_input").unwrap();
        let (hit, max_height) = calculate_trajectory((0, 0), (6, 9), &target);
        assert!(hit);
        assert_eq!(max_height, 45);
    }

    #[test]
    fn test_calculate_max_height_hit_counter() {
        let target = get_target_area("test_input").unwrap();
        let (max_height, hit_counter) = calculate_max_height_total_hit(&target);
        assert_eq!(max_height, 45);
        assert_eq!(hit_counter, 112);
    }
}
