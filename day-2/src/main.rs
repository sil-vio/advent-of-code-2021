use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let lines = lines_from_file("input").expect("Could not load lines");

    let (accx, accy) = &lines[..].into_iter().fold((0,0), |(accx, accy), step| {
        match step.operation {
            StepType::UP => (accx, accy - step.value),
            StepType::DOWN => (accx, accy + step.value),
            StepType::FORWARD => (accx + step.value, accy)
        }
    } );
    println!("accx {}, accy {} = {}", accx, accy, (accx * accy));

    let (accx, accy, _) = &lines[..].into_iter().fold((0,0,0), |(accx, accy, aim), step| {
        match step.operation {
            StepType::UP => (accx, accy , aim - step.value),
            StepType::DOWN => (accx, accy, aim + step.value),
            StepType::FORWARD => (accx + step.value, accy + step.value * aim, aim)
        }
    } );
    println!("accx {}, accy {} = {}", accx, accy, (accx * accy));
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Step>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| x.map(|t| Step::from_str(&t).unwrap()))
        .collect()
}

#[derive(Debug)]
struct Step {
    operation: StepType,
    value: i32,
}

#[derive(Debug)]
enum StepType {
    UP,
    DOWN,
    FORWARD,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_array = s.split(' ').collect::<Vec<&str>>();
        match s_array[0] {
            "up" => Ok(Step {
                operation: StepType::UP,
                value: s_array[1].parse::<i32>().unwrap(),
            }),
            "down" => Ok(Step {
                operation: StepType::DOWN,
                value: s_array[1].parse::<i32>().unwrap(),
            }),
            "forward" => Ok(Step {
                operation: StepType::FORWARD,
                value: s_array[1].parse::<i32>().unwrap(),
            }),
            _ => Err(()),
        }
    }
}
