use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Puzzle {
    nrows: isize,
    ncols: isize,
    antennas: HashMap<char, Vec<(isize, isize)>>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let mut antennas = HashMap::<char, Vec<(isize, isize)>>::new();

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                antennas
                    .entry(c)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }

        let nrows = input.lines().count() as isize;
        let ncols = input.lines().next().unwrap().chars().count() as isize;

        Self {
            nrows,
            ncols,
            antennas,
        }
    }
}

impl Puzzle {
    fn part_1(&self) -> isize {
        let mut antinodes = HashSet::<(isize, isize)>::new();
        for (_, antennas) in &self.antennas {
            for (i, (r0, c0)) in antennas.iter().enumerate() {
                for (r1, c1) in antennas.iter().skip(i + 1) {
                    let ar = 2 * r0 - r1;
                    let ac = 2 * c0 - c1;
                    if ar >= 0 && ar < self.nrows && ac >= 0 && ac < self.ncols {
                        antinodes.insert((ar, ac));
                    }
                    let ar1 = 2 * r1 - r0;
                    let ac1 = 2 * c1 - c0;
                    if ar1 >= 0 && ar1 < self.nrows && ac1 >= 0 && ac1 < self.ncols {
                        antinodes.insert((ar1, ac1));
                    }
                }
            }
        }
        antinodes.len() as isize
    }

    fn part_2(&self) -> isize {
        let mut antinodes = HashSet::<(isize, isize)>::new();
        for (_, antennas) in &self.antennas {
            for (i, (r0, c0)) in antennas.iter().enumerate() {
                for (r1, c1) in antennas.iter().skip(i + 1) {
                    let dr = r0 - r1;
                    let dc = c0 - c1;
                    let mut ar = dr;
                    let mut ac = dc;
                    while r0 - ar >= 0
                        && c0 - ac >= 0
                        && r0 - ar < self.nrows
                        && c0 - ac < self.ncols
                    {
                        antinodes.insert((r0 - ar, c0 - ac));
                        ar += dr;
                        ac += dc;
                    }
                    let mut ar = dr;
                    let mut ac = dc;
                    while r0 + ar >= 0
                        && c0 + ac >= 0
                        && r0 + ar < self.nrows
                        && c0 + ac < self.ncols
                    {
                        antinodes.insert((r0 + ar, c0 + ac));
                        ar += dr;
                        ac += dc;
                    }
                    antinodes.insert((*r0, *c0));
                }
            }
        }
        // for i in 0..self.nrows {
        //     for j in 0..self.ncols {
        //         if antinodes.contains(&(i, j)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        antinodes.len() as isize
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
