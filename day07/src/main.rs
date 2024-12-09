use std::{collections::HashMap, fs};

struct Puzzle {
    input: HashMap<isize, Vec<isize>>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input_raw = fs::read_to_string(file_path).expect("Failed to read the string");
        let input = input_raw
            .lines()
            .map(|line| {
                let (result, nums) = line.split_once(": ").unwrap();
                (
                    result.parse().unwrap(),
                    nums.split(" ").map(|num| num.parse().unwrap()).collect(),
                )
            })
            .collect();

        Self { input }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let mut counter = 0;

        for (total, nums) in &self.input {
            for mut i in 0..2_i32.pow(u32::try_from(nums.len() - 1).unwrap()) {
                let mut nums = nums.iter();
                let mut entry_total = *nums.next().unwrap();
                for num in nums {
                    if i & 1 == 0 {
                        entry_total += *num;
                    } else {
                        entry_total *= *num;
                    }
                    i >>= 1;
                }
                if entry_total == *total {
                    counter += total;
                    break;
                }
            }
        }
        counter
    }

    fn part_2(&mut self) -> isize {
        let mut counter = 0;

        for (total, nums) in &self.input {
            for mut i in 0..3_i32.pow(u32::try_from(nums.len() - 1).unwrap()) {
                let mut nums = nums.iter();
                let mut entry_total = *nums.next().unwrap();
                for num in nums {
                    if i % 3 == 0 {
                        entry_total += *num;
                    } else if i % 3 == 1 {
                        entry_total *= *num;
                    } else {
                        entry_total *= 10_isize.pow(num.ilog10() + 1);
                        entry_total += num;
                    }
                    i /= 3;
                }
                if entry_total == *total {
                    counter += total;
                    break;
                }
            }
        }
        counter
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
