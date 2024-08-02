use std::fs::read_to_string;

fn main() {
    let content =
        read_to_string("C:\\Users\\080576781\\Desktop\\rust\\aoc\\day1\\input.txt").unwrap();

    let mut floor = 0;
    for (count, char) in content.chars().enumerate() {
        match char {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
        if floor == -1 {
            println!("{}", count + 1);
            break;
        }
    }
    println!("{floor}");
}
