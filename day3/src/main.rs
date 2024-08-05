use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let content =
        read_to_string("C:\\Users\\080576781\\Desktop\\rust\\aoc\\day3\\input.txt").unwrap();

    let mut map = HashMap::new();
    let mut santa_previos_point = Point::new(0, 0);
    let mut robo_previous_point = Point::new(0, 0);
    map.insert(santa_previos_point.clone(), 2);
    let mut is_santa = true;

    for char in content.chars() {
        match char {
            '^' => {
                let aux;
                if is_santa {
                    aux = santa_previos_point.new_from_previous(0, 1);
                    santa_previos_point = aux.clone();
                    is_santa = false;
                } else {
                    aux = robo_previous_point.new_from_previous(0, 1);
                    robo_previous_point = aux.clone();
                    is_santa = true;
                }
                map.entry(aux).and_modify(|e| *e += 1).or_insert(1);
            }
            'v' | 'V' => {
                let aux;
                if is_santa {
                    aux = santa_previos_point.new_from_previous(0, -1);
                    santa_previos_point = aux.clone();
                    is_santa = false;
                } else {
                    aux = robo_previous_point.new_from_previous(0, -1);
                    robo_previous_point = aux.clone();
                    is_santa = true;
                }
                map.entry(aux).and_modify(|e| *e += 1).or_insert(1);
            }
            '<' => {
                let aux;
                if is_santa {
                    aux = santa_previos_point.new_from_previous(-1, 0);
                    santa_previos_point = aux.clone();
                    is_santa = false;
                } else {
                    aux = robo_previous_point.new_from_previous(-1, 0);
                    robo_previous_point = aux.clone();
                    is_santa = true;
                }
                map.entry(aux).and_modify(|e| *e += 1).or_insert(1);
            }
            '>' => {
                let aux;
                if is_santa {
                    aux = santa_previos_point.new_from_previous(1, 0);
                    santa_previos_point = aux.clone();
                    is_santa = false;
                } else {
                    aux = robo_previous_point.new_from_previous(1, 0);
                    robo_previous_point = aux.clone();
                    is_santa = true;
                }
                map.entry(aux).and_modify(|e| *e += 1).or_insert(1);
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
