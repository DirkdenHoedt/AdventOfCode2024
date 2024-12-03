use regex::Regex;
use std::fs;

fn main() {
    println!("Solution part 1: {}", part_1());
    println!("Solution part 2: {}", part_2());
}

fn part_1() -> i32 {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Failed to read the string");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for cap in re.captures_iter(input.as_str()) {
        total += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }
    total
}

fn part_2() -> i32 {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Failed to read the string");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut enabled = true;
    for cap in re.captures_iter(input.as_str()) {
        if enabled && cap[0].starts_with("mul") {
            total += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
        } else if cap[0].starts_with("don't(") {
            enabled = false;
        } else if cap[0].starts_with("do(") {
            enabled = true;
        }
    }
    total
}
