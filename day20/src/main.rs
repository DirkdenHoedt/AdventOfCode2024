use std::{collections::HashMap, fs, time::Instant};

use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Pos(i32, i32, i32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Race {
    legal: Vec<Pos>,
    illegal: Vec<Pos>,
    end: Pos,
    start: Pos,
}

enum StepType {
    Legal,
    Illegal,
    Cheat,
}

impl Race {
    fn is_legal(&self, pos: &Pos) -> StepType {
        if self.legal.contains(pos) {
            return StepType::Legal;
        } else if self.illegal.contains(pos) {
            return StepType::Cheat;
        }
        StepType::Illegal
    }
}

struct Puzzle {
    race: Race,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");

        let mut race = Race {
            legal: vec![],
            illegal: vec![],
            end: Pos(0, 0, 0),
            start: Pos(0, 0, 0),
        };
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        race.illegal.push(Pos(i as i32, j as i32, 0));
                    }
                    '.' => {
                        race.legal.push(Pos(i as i32, j as i32, 0));
                    }
                    'S' => {
                        race.start = Pos(i as i32, j as i32, 0);
                        race.legal.push(Pos(i as i32, j as i32, 0));
                    }
                    'E' => {
                        race.end = Pos(i as i32, j as i32, 0);
                        race.legal.push(Pos(i as i32, j as i32, 0));
                    }
                    _ => panic!("Invalid input!"),
                }
            }
        }

        Self { race }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        let x = 0;

        let result = astar(
            &self.race.start.clone(),
            |v| self.successors(v, &self.race),
            |_| x,
            |pos| pos == &self.race.end,
        );
        let (path, _) = result.unwrap();
        let mut route = HashMap::new();

        for (i, p) in path.iter().enumerate() {
            route.insert(*p, i);
        }

        let mut answer = 0;
        for (a, b) in route.keys().tuple_combinations() {
            let Pos(ax, ay, _) = a;
            let Pos(bx, by, _) = b;
            let d = (ax.abs_diff(*bx) + ay.abs_diff(*by)) as usize;
            if route[a].abs_diff(route[b]) >= d + 100 {
                if d <= 2 {
                    answer += 1;
                }
            }
        }
        answer
    }

    fn part_2(&mut self) -> isize {
        let x = 0;

        let result = astar(
            &self.race.start.clone(),
            |v| self.successors(v, &self.race),
            |_| x,
            |pos| pos == &self.race.end,
        );
        let (path, _) = result.unwrap();
        let mut route = HashMap::new();

        for (i, p) in path.iter().enumerate() {
            route.insert(*p, i);
        }

        let mut answer = 0;
        for (a, b) in route.keys().tuple_combinations() {
            let Pos(ax, ay, _) = a;
            let Pos(bx, by, _) = b;
            let d = (ax.abs_diff(*bx) + ay.abs_diff(*by)) as usize;
            if d <= 20 && route[a].abs_diff(route[b]) >= d + 100 {
                answer += 1;
            }
        }
        answer
    }

    fn successors(&self, pos: &Pos, race: &Race) -> Vec<(Pos, i32)> {
        let Pos(x, y, cheats) = pos;
        let directions = vec![Pos(1, 0, 0), Pos(-1, 0, 0), Pos(0, 1, 0), Pos(0, -1, 0)];

        directions
            .into_iter()
            .filter_map(|d| {
                let new_pos = Pos(x + d.0, y + d.1, 0);
                match race.is_legal(&pos) {
                    StepType::Legal => Some((Pos(new_pos.0, new_pos.1, *cheats), 1)),
                    StepType::Illegal => None,
                    StepType::Cheat => {
                        if *cheats == 0 {
                            None
                        } else {
                            Some((Pos(new_pos.0, new_pos.1, cheats - 1), 1))
                        }
                    }
                }
            })
            .collect()
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
        "Solution part 2: {:?}, {:.2?}",
        Puzzle::from("input.txt").part_2(),
        now.elapsed()
    );
}
