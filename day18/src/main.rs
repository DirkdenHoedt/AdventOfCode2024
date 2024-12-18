use std::{fs, time::Instant};

use pathfinding::prelude::astar_bag;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Maze {
    illegal: Vec<Pos>,
    end: Pos,
    start: Pos,
    size: i32,
}

impl Maze {
    fn is_legal(&self, pos: &Pos) -> bool {
        pos.0 >= 0
            && pos.0 <= self.size
            && pos.1 >= 0
            && pos.1 <= self.size
            && !self.illegal.contains(pos)
    }
}

struct Puzzle {
    maze: Maze,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");

        let mut maze = Maze {
            illegal: vec![],
            end: Pos(70, 70),
            start: Pos(0, 0),
            size: 70,
        };
        let illegal = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                Pos(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
        maze.illegal = illegal;

        Self { maze }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        let x = 6;
        self.maze.illegal = self.maze.illegal.clone().into_iter().take(1024).collect();
        let result = astar_bag(
            &self.maze.start.clone(),
            |v| self.successors(v, &self.maze),
            |_| x,
            |pos| pos == &self.maze.end,
        );
        match result {
            Some((_, count)) => {
                return count as isize;
            }
            None => {
                println!("No path found.");
            }
        }
        0
    }

    fn part_2(&mut self) -> Pos {
        let count = self.maze.illegal.len();
        for i in 0..count {
            let illegal = self.maze.illegal.clone();
            self.maze.illegal = self.maze.illegal.clone().into_iter().take(i).collect();
            let x = 6;
            let result = astar_bag(
                &self.maze.start.clone(),
                |v| self.successors(v, &self.maze),
                |_| x,
                |pos| pos == &self.maze.end,
            );
            match result {
                Some((_, _)) => {
                    self.maze.illegal = illegal;
                    continue;
                }
                None => {
                    return illegal[i - 1].clone();
                }
            }
        }
        Pos(-1, -1)
    }

    fn successors(&self, pos: &Pos, maze: &Maze) -> Vec<(Pos, i32)> {
        let Pos(x, y) = pos;
        let directions = vec![Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)];

        directions
            .into_iter()
            .filter_map(|d| {
                let new_pos = Pos(x + d.0, y + d.1);
                if maze.is_legal(&pos) {
                    Some((new_pos, 1))
                } else {
                    None
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
