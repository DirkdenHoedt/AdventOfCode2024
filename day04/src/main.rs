use std::fs;

type Grid = Vec<Vec<char>>;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Puzzle {
    grid: Grid,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }
}

impl Puzzle {
    fn part_1(&self) -> i32 {
        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some('X'))
            .flat_map(|(x, y)| self.words_found(x, y))
            .filter(|found_word| found_word == "XMAS")
            .count() as i32
    }

    fn part_2(&self) -> i32 {
        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some('A'))
            .filter(|(x, y)| {
                let lt = self.char_at(x - 1, y - 1);
                let rt = self.char_at(x + 1, y - 1);
                let lb = self.char_at(x - 1, y + 1);
                let rb = self.char_at(x + 1, y + 1);

                matches!(
                    (lt, rb, lb, rt),
                    (Some('M'), Some('S'), Some('M'), Some('S'))
                        | (Some('S'), Some('M'), Some('S'), Some('M'))
                        | (Some('M'), Some('S'), Some('S'), Some('M'))
                        | (Some('S'), Some('M'), Some('M'), Some('S'))
                )
            })
            .count() as i32
    }

    fn words_found(&self, x: isize, y: isize) -> impl Iterator<Item = String> + '_ {
        DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
            (0..4)
                .map(|n| self.char_at(x + dx * n, y + dy * n))
                .collect()
        })
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn positions(&self) -> impl Iterator<Item = (isize, isize)> {
        let x_size = self.grid.len();
        let y_size = self.grid[0].len();
        (0..x_size).flat_map(move |x| (0..y_size).map(move |y| (x as isize, y as isize)))
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
