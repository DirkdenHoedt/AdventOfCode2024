use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Solution part 1: {}", part_1());
    println!("Solution part 2: {}", part_2());
}

fn part_1() -> i32 {
    let file_path = "input.txt";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        let dir = numbers[0] < numbers[1];
        let mut correct = true;

        for nums in numbers.windows(2) {
            let gap = (nums[0] - nums[1]).abs();
            if dir && (nums[0] > nums[1] || gap < 1 || gap > 3) {
                correct = false;
            } else if !dir && (nums[0] < nums[1] || gap < 1 || gap > 3) {
                correct = false;
            }
        }

        if correct {
            counter += 1;
        }
    }
    counter
}

fn part_2() -> i32 {
    let file_path = "input.txt";

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        let dir = numbers[0] < numbers[1];
        let mut correct = true;

        for nums in numbers.windows(2) {
            let gap = (nums[0] - nums[1]).abs();
            if dir && (nums[0] > nums[1] || gap < 1 || gap > 3) {
                correct = false;
            } else if !dir && (nums[0] < nums[1] || gap < 1 || gap > 3) {
                correct = false;
            }
        }

        if !correct {
            for n in 0..numbers.len() {
                let mut numbers2 = numbers.clone();
                numbers2.remove(n);
                let mut correct2 = true;
                let dir = numbers2[0] < numbers2[1];
                for nums in numbers2.windows(2) {
                    let gap = (nums[0] - nums[1]).abs();
                    if dir && (nums[0] > nums[1] || gap < 1 || gap > 3) {
                        correct2 = false;
                    } else if !dir && (nums[0] < nums[1] || gap < 1 || gap > 3) {
                        correct2 = false;
                    }
                }
                if correct2 {
                    correct = true;
                    break;
                }
            }
        }

        if correct {
            counter += 1;
        }
    }
    counter
}
