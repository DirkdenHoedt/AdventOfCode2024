use std::{fs, time::Instant};

use regex::Regex;

struct Puzzle {
    robots: Vec<((isize, isize), (isize, isize))>,
    width: isize,
    height: isize,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");

        let re = Regex::new(
            r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)",
        )
        .unwrap();
        let mut robots = Vec::new();

        for cap in re.captures_iter(&input) {
            robots.push((
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            ));
        }

        Self { robots, width: 101, height: 103 }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        for _ in 0..100 {
            for ((px, py), (vx, vy)) in self.robots.iter_mut() {
                *px = (*px + self.width + *vx) % self.width;
                *py = (*py + self.height + *vy) % self.height;
            }
        }
        
        let q1: isize = self.robots.iter().filter_map(|((x, y), _)| {
            if *x < self.width / 2 && *y < self.height / 2 {
                Some(1)
            } else {
                None
            }
        }).sum();
        let q2: isize = self.robots.iter().filter_map(|((x, y), _)| {
            if *x > self.width / 2 && *y < self.height / 2 {
                Some(1)
            } else {
                None
            }
        }).sum();
        let q3: isize = self.robots.iter().filter_map(|((x, y), _)| {
            if *x < self.width / 2 && *y > self.height / 2 {
                Some(1)
            } else {
                None
            }
        }).sum();
        let q4: isize = self.robots.iter().filter_map(|((x, y), _)| {
            if *x > self.width / 2 && *y > self.height / 2 {
                Some(1)
            } else {
                None
            }
        }).sum();
        q1 * q2 * q3 * q4
    }

    fn part_2(&mut self) -> isize {
        for round in 0..10000 {
            for ((px, py), (vx, vy)) in self.robots.iter_mut() {
                *px = (*px + self.width + *vx) % self.width;
                *py = (*py + self.height + *vy) % self.height;
            }
            
            let std_x = self.std_deviation(&self.robots.iter().map(|((x, _), _)| *x).collect()).unwrap_or(100.0);
            let std_y = self.std_deviation(&self.robots.iter().map(|((_, y), _)| *y).collect()).unwrap_or(100.0);
            if std_x < 25.0 && std_y < 25.0 {
                // let coords = self.robots.iter().map(|((x, y), _)| (x, y)).collect::<Vec<_>>();
                // for i in 0..self.height {
                //     for j in 0..self.width {
                //         if coords.contains(&(&j, &i)) {
                //             print!("#");
                //         } else {
                //             print!(".")
                //         }
                //     }
                //     println!();
                // }
                // println!();
                return round + 1;
            }
        }
        0
    }

    fn mean(&self, data: &Vec<isize>) -> Option<f32> {
        let sum = data.iter().sum::<isize>() as f32;
        let count = data.len();
    
        match count {
            positive if positive > 0 => Some(sum / count as f32),
            _ => None,
        }
    }
    
    fn std_deviation(&self, data: &Vec<isize>) -> Option<f32> {
        match (self.mean(data), data.len()) {
            (Some(data_mean), count) if count > 0 => {
                let variance = data.iter().map(|value| {
                    let diff = data_mean - (*value as f32);
    
                    diff * diff
                }).sum::<f32>() / count as f32;
    
                Some(variance.sqrt())
            },
            _ => None
        }
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
