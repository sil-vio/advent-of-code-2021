use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    horizontal: bool,
    vertical: bool,
    diagonal: bool,
}

impl Line {
    fn points_covered(&self, only_hor_ver: bool) -> Vec<Point> {
        if only_hor_ver && (self.horizontal || self.vertical)
            || !only_hor_ver && (self.horizontal || self.vertical || self.diagonal)
        {
            self.get_points_covered()
        } else {
            Vec::new()
        }
    }

    fn get_points_covered(&self) -> Vec<Point> {

        let mut points: Vec<Point> = Vec::new();
        let mut distance_x = (self.start.x - self.end.x).abs() ;
        let mut distance_y = (self.start.y - self.end.y).abs() ;
        let direction_x = if self.start.x <= self.end.x { 1 } else { -1 };
        let direction_y = if self.start.y <= self.end.y { 1 } else { -1 };
        while distance_x >= 0 || distance_y >= 0 {

            let point_x = if distance_x >= 0 { distance_x * direction_x } else { 0 };
            let point_y = if distance_y >= 0 { distance_y * direction_y } else { 0 };

            points.push(Point {
                x: self.start.x + point_x,
                y: self.start.y + point_y,
            });
            distance_x -= 1;
            distance_y -= 1;
        }
        points
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .map(|(x, y)| (Point::from_str(x).unwrap(), Point::from_str(y).unwrap()))
            .unwrap();
        let is_diagonal = (start.x - end.x).abs() == (start.y - end.y).abs();
        Ok(Line {
            start,
            end,
            horizontal: start.x != end.x && start.y == end.y,
            vertical: start.y != end.y && start.x == end.x,
            diagonal: is_diagonal,
        })
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();
        Ok(Point { x, y })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut graph_point: HashMap<Point, u16> = HashMap::new();

    let mut graph: Vec<Line> = Vec::new();
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data) = line {
                graph.push(Line::from_str(&data).unwrap());
            }
        }
    }
    for i in 0..graph.len() {
        let line_point = graph[i].points_covered(true);
        line_point.iter().for_each(|e| {
            let value = graph_point.entry(Point { x: e.x, y: e.y }).or_insert(0);
            *value += 1;
        });
    }
    println!(
        "result {:?}",
        graph_point.iter().filter(|e| e.1 > &1).count()
    );

    let mut graph_points_2: HashMap<Point, u16> = HashMap::new();
    for i in 0..graph.len() {
        let line_point = graph[i].points_covered(false);
        line_point.iter().for_each(|e| {
            let value = graph_points_2.entry(Point { x: e.x, y: e.y }).or_insert(0);
            *value += 1;
        });
    }

    println!(
        "result {:?}",
        graph_points_2.iter().filter(|e| e.1 > &1).count()
    );
}
