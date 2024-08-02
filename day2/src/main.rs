use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "C:\\Users\\080576781\\Desktop\\rust\\aoc\\day2\\input.txt";
    // let content =
    //     read_to_string("C:\\Users\\080576781\\Desktop\\rust\\aoc\\day2\\input.txt").unwrap();

    let file = File::open(path).unwrap();
    let content = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in content {
        let parsed: Vec<u32> = line
            .unwrap()
            .split("x")
            .map(|val| val.parse().unwrap())
            .collect();

        let l = parsed[0];
        let w = parsed[1];
        let h = parsed[2];
        let mut results = [2 * l * w, 2 * w * h, 2 * h * l];
        results.sort_unstable();
        println!("{:?}", results);
        sum += results[0] / 2 + results.into_iter().sum::<u32>();
    }
    println!("{sum}")
}
