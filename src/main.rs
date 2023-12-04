use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn find_first_digit(s: String) -> Option<char> {
    s.chars().find(|&c| c.is_digit(10))
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    //let tests = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    //let tests = lines_from_file("day1input.txt");
    let tests = lines_from_file("day1part2tests.txt");
    let mut total = 0;
    for test in tests {
        let first = find_first_digit(test.to_string()).unwrap();
        let last = find_first_digit(test.chars().rev().collect::<String>()).unwrap();
        let number = format!("{first}{last}");
        let val: i32 = number.parse().unwrap();
        total += val;
    }
    println!("{total}");
}
