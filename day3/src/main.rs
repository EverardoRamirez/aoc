use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let content =
        read_to_string("C:\\Users\\080576781\\Desktop\\rust\\aoc\\day3\\input.txt").unwrap();

    let mut map = HashMap::new();
    let mut previos_point = Point::new(0, 0);
    map.insert(previos_point.clone(), 1);

    for char in content.chars() {
        match char {
            '^' => {
                previos_point = previos_point.new_from_previous(0, 1);
                map.entry(previos_point.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            'v' | 'V' => {
                previos_point = previos_point.new_from_previous(0, -1);
                map.entry(previos_point.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            '<' => {
                previos_point = previos_point.new_from_previous(-1, 0);
                map.entry(previos_point.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            '>' => {
                previos_point = previos_point.new_from_previous(1, 0);
                map.entry(previos_point.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            _ => {}
        }
    }

    println!("{:?}", map);
    println!("{}", map.len());
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn new_from_previous(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
