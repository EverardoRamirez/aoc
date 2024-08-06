use fancy_regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let path = "C:\\Users\\080576781\\Desktop\\rust\\aoc\\day5\\input.txt";
    // let content =
    //     read_to_string("C:\\Users\\080576781\\Desktop\\rust\\aoc\\day2\\input.txt").unwrap();

    let file = File::open(path).unwrap();
    let content = BufReader::new(file).lines();
    let mut counter = 0;
    for line in content {
        let line = line.unwrap();
        if has_3_vowels_or_more(&line) && has_twice_in_arow(&line) && !contains_prohibited(&line) {
            println!("{line} is nice");
            counter += 1;
        } else {
            println!("{line} is nasty");
        }
    }
    println!("{counter}");
}

fn has_3_vowels_or_more(line: &str) -> bool {
    let re = Regex::new(r"(.*(a|e|i|o|u).*){3,}").unwrap();
    re.is_match(line).unwrap()
}

fn has_twice_in_arow(line: &str) -> bool {
    let re = Regex::new(r".*(.)\1{1,}.*").unwrap();
    re.is_match(line).unwrap()
}

fn contains_prohibited(line: &str) -> bool {
    let re = Regex::new(r".*(ab)|(cd)|(pq)|(xy).*").unwrap();
    re.is_match(line).unwrap()
}
