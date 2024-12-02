use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    println!("Result part 1: {}", part_1());
    println!("Result part 2: {}", part_2());
}

fn part_1() -> i32 {
    let file_path = "input.txt";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut list_a = vec![];
    let mut list_b = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse().expect("Failed to parse number")) // Parse each part as i32
            .collect();

        list_a.push(numbers[0]);
        list_b.push(numbers[1]);
    }

    list_a.sort();
    list_b.sort();

    let res: i32 = list_a.iter().zip(list_b.iter()).map(|(a, b)| (b - a).abs()).sum();

    res
}

fn part_2() -> i32 {
    let file_path = "input.txt";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut list_a = vec![];
    let mut list_b = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse().expect("Failed to parse number")) // Parse each part as i32
            .collect();

        list_a.push(numbers[0]);
        list_b.push(numbers[1]);
    }

    let res: i32 = list_a.iter().map(|a| list_b.iter().filter(|b| *b == a).count() as i32 * a).sum();

    res
}