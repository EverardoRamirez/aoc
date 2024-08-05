fn main() {
    let input: &str = "bgvyzdsv";
    let mut count: i128 = 0;
    loop {
        let test = format!("{input}{count}");
        let digest = format!("{:x}", md5::compute(test.as_bytes()));
        if digest.starts_with("000000") {
            println!("{count}");
            break;
        }
        count += 1;
    }

    println!("Hello, world!");
}
