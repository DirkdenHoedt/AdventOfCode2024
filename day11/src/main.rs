use std::{collections::HashMap, fs};

struct Puzzle {
    stones: Vec<i64>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let stones = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        Self { stones }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        for _ in 0..25 {
            let mut new_stones = Vec::new();
            for stone in &self.stones {
                if *stone == 0 {
                    new_stones.push(1);
                    continue;
                }

                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let num1 = *stone / 10_i64.pow(digits / 2);
                    let num2 = *stone - num1 * 10_i64.pow(digits / 2);
                    new_stones.push(num1);
                    new_stones.push(num2);
                    continue;
                }

                new_stones.push(*stone * 2024);
            }
            // println!("{:?}", self.stones);
            self.stones = new_stones;
        }
        self.stones.len() as isize
    }

    fn part_2(&mut self) -> isize {
        let mut stones = HashMap::<i64, i64>::new();
        self.stones
            .iter()
            .for_each(|stone| *stones.entry(*stone).or_insert(0) += 1);

        for _ in 0..75 {
            let mut new_stones = HashMap::<i64, i64>::new();
            for (stone, stone_count) in &stones {
                if *stone == 0 {
                    *new_stones.entry(1).or_insert(0) += *stone_count;
                    continue;
                }

                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let num1 = *stone / 10_i64.pow(digits / 2);
                    let num2 = *stone - num1 * 10_i64.pow(digits / 2);
                    *new_stones.entry(num1).or_insert(0) += *stone_count;
                    *new_stones.entry(num2).or_insert(0) += *stone_count;
                    continue;
                }

                *new_stones.entry(*stone * 2024).or_insert(0) += *stone_count;
            }
            // println!("{} {}", i, stones.len());
            stones = new_stones;
        }
        stones
            .iter()
            .map(|(_, stone_count)| *stone_count)
            .sum::<i64>() as isize
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
