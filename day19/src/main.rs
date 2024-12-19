use std::{collections::HashMap, fs, time::Instant};

struct Puzzle {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let (patterns, designs) = input.split_once("\r\n\r\n").unwrap();
        let patterns = patterns.split(", ").map(|s| s.to_string()).collect();
        let designs = designs.lines().map(|s| s.to_string()).collect();

        Self { patterns, designs }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        let mut counter = 0;
        let mut memo = HashMap::new();

        for design in &self.designs {
            if self.can_form_string(design, &mut memo) != 0 {
                counter += 1;
            }
        }
        counter
    }

    fn part_2(&mut self) -> isize {
        let mut counter = 0;
        let mut memo = HashMap::new();

        for design in &self.designs {
            counter += self.can_form_string(design, &mut memo);
        }
        counter
    }

    fn can_form_string(&self, target: &String, memo: &mut HashMap<String, isize>) -> isize {
        if let Some(&result) = memo.get(target) {
            return result;
        }

        if target.is_empty() {
            return 1;
        }

        let mut variations = 0;
        for sub in &self.patterns {
            if target.starts_with(sub) {
                let remaining = &target[sub.len()..].to_string();
                variations += self.can_form_string(remaining, memo);
            }
        }
        memo.insert(target.to_string(), variations);
        variations
    }
}

fn main() {
    let now = Instant::now();
    println!(
        "Solution part 1: {}, {:.2?}",
        Puzzle::from("input.txt").part_1(),
        now.elapsed()
    );
    let now = Instant::now();
    println!(
        "Solution part 2: {}, {:.2?}",
        Puzzle::from("input.txt").part_2(),
        now.elapsed()
    );
}
