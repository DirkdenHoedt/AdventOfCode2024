use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Grid = Vec<Vec<char>>;

struct Puzzle {
    grid: Grid,
    turns: HashMap<(isize, isize), (isize, isize)>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

        let turns: HashMap<(isize, isize), (isize, isize)> = HashMap::from([
            ((1, 0), (0, -1)),
            ((0, -1), (-1, 0)),
            ((-1, 0), (0, 1)),
            ((0, 1), (1, 0)),
        ]);

        Self { grid, turns }
    }
}

impl Puzzle {
    fn part_1(&self) -> i32 {
        let (mut x, mut y) = self.find_char('^').unwrap();
        let mut dir = (-1, 0);
        let mut pos = HashSet::new();
        pos.insert((x, y));

        while self.char_at(x + dir.0, y + dir.1) != None {
            if self.char_at(x + dir.0, y + dir.1) == Some('#') {
                dir = self.turns[&dir];
            }
            x += dir.0;
            y += dir.1;
            pos.insert((x, y));
            // println!("{}, {}", x, y);
        }
        // for i in (0..10) {
        //     for j in (0..10) {
        //         if pos.contains(&(i, j)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
        pos.len() as i32
    }

    fn part_2(&mut self) -> i32 {
        let (mut x, mut y) = self.find_char('^').unwrap();
        let mut dir = (-1, 0);
        let mut pos = HashSet::new();
        // pos.insert((x, y));

        while self.char_at(x + dir.0, y + dir.1) != None {
            if self.char_at(x + dir.0, y + dir.1) == Some('#') {
                dir = self.turns[&dir];
            }
            x += dir.0;
            y += dir.1;
            pos.insert((x, y));
        }

        pos.iter()
            .filter(|(i, j)| {
                self.grid[*i as usize][*j as usize] = '#';
                let ok = self.simulate();
                self.grid[*i as usize][*j as usize] = '.';
                ok
            })
            .count() as i32
    }

    fn simulate(&self) -> bool {
        let (mut x, mut y) = self.find_char('^').unwrap();
        let mut dir = (-1, 0);
        let mut pos = HashSet::new();
        // pos.insert((x, y, dir));

        while self.char_at(x + dir.0, y + dir.1) != None {
            if self.char_at(x + dir.0, y + dir.1) == Some('#') {
                dir = self.turns[&dir];
            }
            x += dir.0;
            y += dir.1;
            if pos.contains(&(x, y, dir)) {
                return true;
            }
            pos.insert((x, y, dir));
        }
        false
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn find_char(&self, target: char) -> Option<(isize, isize)> {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch == target {
                    return Some((i as isize, j as isize));
                }
            }
        }
        None
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
