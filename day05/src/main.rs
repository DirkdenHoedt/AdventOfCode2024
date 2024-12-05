use std::fs;

use hashbrown::{HashMap, HashSet};

struct Puzzle {
    order: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let binding = fs::read_to_string(file_path).expect("Failed to read the string");
        let (p1, p2) = binding.split_once("\r\n\r\n").unwrap();

        let mut order = HashMap::<usize, HashSet<usize>>::new();

        for line in p1.lines() {
            let (before, after) = line.split_once("|").unwrap();
            order
                .entry(after.parse().unwrap())
                .or_default()
                .insert(before.parse().unwrap());
        }

        let updates: Vec<Vec<usize>> = p2
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Self { order, updates }
    }
}

impl Puzzle {
    fn part_1(&self) -> usize {
        let mut counter = 0;
        for update in &self.updates {
            if update.is_sorted_by(|a, b| self.order[b].contains(a)) {
                counter += update[update.len() / 2];
            }
        }
        counter
    }

    fn part_2(&self) -> usize {
        let mut counter = 0;
        for mut update in self.updates.clone() {
            if !update.is_sorted_by(|a, b| self.order[b].contains(a)) {
                update.sort_by(|a, b| self.order[b].contains(a).cmp(&true));
                counter += update[update.len() / 2];
            }
        }
        counter
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
