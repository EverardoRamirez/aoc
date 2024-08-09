use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const SIDE: usize = 1000;

fn main() {
    let path = "C:\\Users\\080576781\\Desktop\\rust\\aoc\\day6\\input.txt";

    let file = File::open(path).unwrap();
    let content = BufReader::new(file).lines();

    let mut matrix = [[false; SIDE]; SIDE];
    for line in content {
        let line = line.unwrap();

        if line.starts_with("turn on") {
            let points = Point::from_str(&line["turn on ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| *x = true);
        } else if line.starts_with("turn off") {
            let points = Point::from_str(&line["turn off ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| *x = false);
        } else if line.starts_with("toggle") {
            let points = Point::from_str(&line["toggle ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| *x = !*x);
        }
    }

    let count = matrix.iter().flatten().filter(|x| **x).count();
    matrix.iter().for_each(|x| println!("{:?}", x));

    println!("{count}");
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(line: &str) -> (Self, Self) {
        let (start, end) = line.split_once("through").unwrap();
        println!("start {start} end {end}");
        (Self::from_coordinates(start), Self::from_coordinates(end))
    }

    fn from_coordinates(line: &str) -> Self {
        let (x, y) = line.split_once(",").unwrap();
        Self {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        }
    }
}

fn update_light(
    array: &mut [[bool; SIDE]; SIDE],
    from: &Point,
    to: &Point,
    command: fn(&mut bool) -> (),
) {
    let rows = &mut array[from.y..=to.y];
    for row in rows {
        let colums = &mut row[from.x..=to.x];
        for ligth in colums {
            command(ligth);
        }
    }
}
