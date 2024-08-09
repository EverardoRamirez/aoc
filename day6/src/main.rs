use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const SIDE: usize = 1000;

fn main() {
    let path = "C:\\Users\\080576781\\Desktop\\rust\\aoc\\day6\\input.txt";

    let file = File::open(path).unwrap();
    let content = BufReader::new(file).lines();

    let mut matrix = vec![[0_u32; SIDE]; SIDE];
    for line in content {
        let line = line.unwrap();

        if line.starts_with("turn on") {
            let points = Point::from_str(&line["turn on ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| *x += 1);
        } else if line.starts_with("turn off") {
            let points = Point::from_str(&line["turn off ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| {
                if *x != 0 {
                    *x -= 1
                }
            });
        } else if line.starts_with("toggle") {
            let points = Point::from_str(&line["toggle ".len()..]);
            update_light(&mut matrix, &points.0, &points.1, |x| *x += 2);
        }
    }

    let count = matrix
        .into_iter()
        .flatten()
        .reduce(|prev, current| prev + current)
        .unwrap();

    println!("{}", count);
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

fn update_light(array: &mut [[u32; SIDE]], from: &Point, to: &Point, command: fn(&mut u32) -> ()) {
    let rows = &mut array[from.y..=to.y];
    for row in rows {
        let colums = &mut row[from.x..=to.x];
        for ligth in colums {
            command(ligth);
        }
    }
}
