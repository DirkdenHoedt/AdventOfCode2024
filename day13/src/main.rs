use std::{fs, time::Instant};

use regex::Regex;

struct Puzzle {
    machines: Vec<((isize, isize), (isize, isize), (isize, isize))>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");

        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\r\nButton B: X\+(\d+), Y\+(\d+)\r\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();
        let mut machines = Vec::new();

        for cap in re.captures_iter(&input) {
            machines.push((
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
                (cap[5].parse().unwrap(), cap[6].parse().unwrap()),
            ));
        }

        Self { machines }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let mut tokens = 0;
        for ((ax, ay), (bx, by), (px, py)) in &self.machines {
            let mut results = Vec::new();
            for ba in 0..100 {
                for bb in 0..100 {
                    if ba * *ax + bb * *bx == *px && ba * *ay + bb * *by == *py {
                        results.push((ba, bb));
                    }
                }
            }
            let (ra, rb) = results.iter().min_by_key(|x| x.0).unwrap_or(&(0, 0));
            tokens += ra * 3 + rb;
        }
        tokens
    }

    fn part_2(&self) -> isize {
        let mut tokens = 0;
        for ((ax, ay), (bx, by), (px, py)) in &self.machines {
            let px = px + 10000000000000;
            let py = py + 10000000000000;

            let (ra, rb) = self
                .solve_scaling_factors((*ax, *ay), (*bx, *by), (px, py))
                .unwrap_or((0, 0));
            tokens += ra * 3 + rb;
        }
        tokens
    }

    fn solve_scaling_factors(
        &self,
        a: (isize, isize),
        b: (isize, isize),
        p: (isize, isize),
    ) -> Option<(isize, isize)> {
        let (ax, ay) = a;
        let (bx, by) = b;
        let (px, py) = p;

        let num_a = py * bx - px * by;
        let denom_a = ay * bx - ax * by;
        if num_a % denom_a != 0 {
            return None;
        }

        let num_b = py * ax - px * ay;
        let denom_b = -denom_a;
        if num_b % denom_b != 0 {
            return None;
        }

        Some((num_a / denom_a, num_b / denom_b))
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
