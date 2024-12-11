use std::{collections::HashSet, fs};

type Grid = Vec<Vec<i8>>;

struct Puzzle {
    grid: Grid,
    directions: Vec<(isize, isize)>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let grid: Grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect()
            })
            .collect();

        let directions = Vec::from([(0, 1), (1, 0), (-1, 0), (0, -1)]);

        Self { grid, directions }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let mut counter = 0;
        let starts = self.find_nums(0);
        for (sx, sy) in starts {
            let mut ends = HashSet::new();
            for (dx, dy) in &self.directions {
                self.traverse_grid_set(sx + dx, sy + dy, 0, &mut ends);
            }
            counter += ends.len();
        }

        counter as isize
    }

    fn part_2(&self) -> isize {
        let mut counter = 0;
        let starts = self.find_nums(0);
        for (sx, sy) in starts {
            for (dx, dy) in &self.directions {
                counter += self.traverse_grid(sx + dx, sy + dy, 0);
            }
        }

        counter as isize
    }

    fn traverse_grid(&self, x: isize, y: isize, num: i8) -> isize {
        let new_num = self.num_at(x, y);
        if new_num == None {
            return 0;
        }
        let new_num = new_num.unwrap();
        if new_num - num == 1 {
            if new_num == 9 {
                return 1;
            } else {
                let mut res = 0;
                for (dx, dy) in &self.directions {
                    res += self.traverse_grid(x + dx, y + dy, new_num);
                }
                return res;
            }
        } else {
            return 0;
        }
    }

    fn traverse_grid_set(&self, x: isize, y: isize, num: i8, ends: &mut HashSet<(isize, isize)>) {
        let new_num = self.num_at(x, y);
        if new_num == None {
            return;
        }
        let new_num = new_num.unwrap();
        if new_num - num == 1 {
            if new_num == 9 {
                ends.insert((x, y));
                return;
            } else {
                for (dx, dy) in &self.directions {
                    self.traverse_grid_set(x + dx, y + dy, new_num, ends);
                }
            }
        } else {
            return;
        }
    }

    fn num_at(&self, x: isize, y: isize) -> Option<i8> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn find_nums(&self, target: i8) -> Vec<(isize, isize)> {
        let mut res = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == target {
                    res.push((i as isize, j as isize));
                }
            }
        }
        res
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
