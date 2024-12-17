use std::{fs, time::Instant};

use pathfinding::{directed::dijkstra, prelude::astar_bag};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ReindeerArena {
    legal: Vec<Pos>,
    illegal: Vec<Pos>,
    end: Pos,
    start: Pos,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Vector {
    pos: Pos,
    dir: Pos,
}

impl ReindeerArena {
    fn is_legal(&self, pos: &Pos) -> bool {
        self.legal.contains(pos) && !self.illegal.contains(pos)
    }
}

struct Puzzle {
    arena: ReindeerArena,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");

        let mut arena = ReindeerArena {
            legal: vec![],
            illegal: vec![],
            end: Pos(0, 0),
            start: Pos(0, 0),
        };
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        arena.illegal.push(Pos(i as i32, j as i32));
                    }
                    '.' => {
                        arena.legal.push(Pos(i as i32, j as i32));
                    }
                    'S' => {
                        arena.start = Pos(i as i32, j as i32);
                        arena.legal.push(Pos(i as i32, j as i32));
                    }
                    'E' => {
                        arena.end = Pos(i as i32, j as i32);
                        arena.legal.push(Pos(i as i32, j as i32));
                    }
                    _ => panic!("Invalid input!"),
                }
            }
        }

        Self { arena }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let (_, result) = dijkstra::dijkstra(
            &Vector {
                pos: self.arena.start.clone(),
                dir: Pos(0, 0),
            },
            |v| self.successors(v, &self.arena),
            |v| v.pos == self.arena.end,
        )
        .unwrap();
        result as isize
    }

    fn part_2(&self) -> isize {
        let x = 0;
        let result = astar_bag(
            &Vector {
                pos: self.arena.start.clone(),
                dir: Pos(0, 0),
            },
            |v| self.successors(v, &self.arena),
            |_| x,
            |v| v.pos == self.arena.end,
        );
        let mut seats: Vec<Pos> = vec![];
        match result {
            Some((path, _)) => {
                for p in path {
                    seats.extend(p.iter().map(|v| v.pos.clone()).collect::<Vec<Pos>>());
                }
            }
            None => {
                println!("No path found.");
            }
        }
        seats.sort();
        seats.dedup();
        seats.len() as isize
    }

    fn successors(&self, v: &Vector, arena: &ReindeerArena) -> Vec<(Vector, i32)> {
        let Pos(x, y) = v.pos;
        let directions = vec![Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)];

        directions
            .into_iter()
            .filter_map(|d| {
                let new_pos = Pos(x + d.0, y + d.1);
                let new_dir = Pos(d.0, d.1);
                let new_v = Vector {
                    pos: new_pos,
                    dir: new_dir,
                };
                if arena.is_legal(&new_v.pos) {
                    let score = if &d == &v.dir { 1 } else { 1001 };
                    Some((new_v, score))
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
        "Solution part 2: {}, {:.2?}",
        Puzzle::from("input.txt").part_2(),
        now.elapsed()
    );
}
