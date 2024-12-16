use std::{char, collections::HashMap, fs};

type Grid = Vec<Vec<char>>;

struct Puzzle {
    grid: Grid,
    steps: Vec<char>,
    directions: HashMap<char, (isize, isize)>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let (maze, steps_raw) = input.split_once("\r\n\r\n").unwrap();

        let grid: Grid = maze.lines().map(|line| line.chars().collect()).collect();

        let steps = steps_raw.lines().flat_map(|line| line.chars()).collect();
        let directions =
            HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

        Self {
            grid,
            steps,
            directions,
        }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        let (mut rx, mut ry) = self.find_char('@').unwrap();
        let steps = self.steps.clone();

        for step in steps.iter() {
            let (dx, dy) = self.directions[step];
            if self.move_object(rx, ry, rx + dx, ry + dy) {
                self.set_char(rx, ry, '.');
                self.set_char(rx + dx, ry + dy, '@');
                rx += dx;
                ry += dy;
            }
        }
        // for line in &self.grid {
        //     for char in line {
        //         print!("{}", char);
        //     }
        //     println!();
        // }
        // println!();
        let mut res = 0;
        for (i, line) in self.grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == 'O' {
                    res += i * 100 + j;
                }
            }
        }
        res as isize
    }

    fn part_2(&mut self) -> isize {
        let mut new_grid = Grid::new();
        for line in &self.grid {
            let mut new_line = Vec::new();
            for c in line {
                match c {
                    '#' => {
                        new_line.push('#');
                        new_line.push('#');
                    }
                    '.' => {
                        new_line.push('.');
                        new_line.push('.');
                    }
                    'O' => {
                        new_line.push('[');
                        new_line.push(']');
                    }
                    '@' => {
                        new_line.push('@');
                        new_line.push('.');
                    }
                    _ => panic!("Unknown input"),
                }
            }
            new_grid.push(new_line);
        }
        self.grid = new_grid;

        let (mut rx, mut ry) = self.find_char('@').unwrap();
        let steps = self.steps.clone();

        for step in steps.iter() {
            let (dx, dy) = self.directions[step];
            if self.move_big_object(rx, ry, rx + dx, ry + dy) {
                self.set_char(rx, ry, '.');
                self.set_char(rx + dx, ry + dy, '@');
                rx += dx;
                ry += dy;
            }
        }

        // for line in &self.grid {
        //     for char in line {
        //         print!("{}", char);
        //     }
        //     println!();
        // }
        // println!();

        let mut res = 0;
        for (i, line) in self.grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '[' {
                    res += i * 100 + j;
                }
            }
        }
        res as isize
    }

    fn move_big_object(&mut self, sx: isize, sy: isize, dx: isize, dy: isize) -> bool {
        match self.char_at(dx, dy) {
            Some('#') => return false,
            Some('.') => {
                self.set_char(dx, dy, self.char_at(sx, sy).unwrap());
                self.set_char(sx, sy, '.');
                return true;
            }
            Some('[') => {
                if dx - sx == 0 {
                    if self.move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy)) {
                        self.set_char(dx, dy, ']');
                        self.set_char(sx, sy, '.');
                        return true;
                    }
                    return false;
                } else {
                    if !self.try_move_big_object(sx, sy, dx, dy) {
                        return false;
                    }
                    if self.move_big_object(dx, dy + 1, dx + (dx - sx), dy + (dy - sy) + 1)
                        && self.move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy))
                    {
                        // self.set_char(dx, dy + 1, self.char_at(sx, sy + 1).unwrap());
                        self.set_char(dx, dy, self.char_at(sx, sy).unwrap());
                        self.set_char(sx, sy, '.');
                        // self.set_char(sx, sy + 1, '.');
                        return true;
                    }
                    return false;
                }
            }
            Some(']') => {
                if dx - sx == 0 {
                    if self.move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy)) {
                        self.set_char(dx, dy, '[');
                        self.set_char(sx, sy, '.');
                        return true;
                    }
                    return false;
                } else {
                    if !self.try_move_big_object(sx, sy, dx, dy) {
                        return false;
                    }
                    if self.move_big_object(dx, dy - 1, dx + (dx - sx), dy + (dy - sy) - 1)
                        && self.move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy))
                    {
                        // self.set_char(dx, dy - 1, self.char_at(sx, sy - 1).unwrap());
                        self.set_char(dx, dy, self.char_at(sx, sy).unwrap());
                        self.set_char(sx, sy, '.');
                        // self.set_char(sx, sy - 1, '.');
                        return true;
                    }
                    return false;
                }
            }
            _ => return false,
        }
    }

    fn try_move_big_object(&mut self, sx: isize, sy: isize, dx: isize, dy: isize) -> bool {
        match self.char_at(dx, dy) {
            Some('#') => return false,
            Some('.') => {
                return true;
            }
            Some('[') => {
                if dx - sx == 0 {
                    if self.try_move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy)) {
                        return true;
                    }
                    return false;
                } else {
                    if self.try_move_big_object(dx, dy + 1, dx + (dx - sx), dy + (dy - sy) + 1)
                        && self.try_move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy))
                    {
                        return true;
                    }
                    return false;
                }
            }
            Some(']') => {
                if dx - sx == 0 {
                    if self.try_move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy)) {
                        return true;
                    }
                    return false;
                } else {
                    if self.try_move_big_object(dx, dy - 1, dx + (dx - sx), dy + (dy - sy) - 1)
                        && self.try_move_big_object(dx, dy, dx + (dx - sx), dy + (dy - sy))
                    {
                        return true;
                    }
                    return false;
                }
            }
            _ => return false,
        }
    }

    fn move_object(&mut self, sx: isize, sy: isize, dx: isize, dy: isize) -> bool {
        match self.char_at(dx, dy) {
            Some('#') => return false,
            Some('.') => {
                self.set_char(dx, dy, self.char_at(sx, sy).unwrap());
                self.set_char(sx, sy, '.');
                return true;
            }
            Some('O') => {
                if self.move_object(dx, dy, dx + (dx - sx), dy + (dy - sy)) {
                    self.set_char(dx, dy, self.char_at(sx, sy).unwrap());
                    self.set_char(sx, sy, '.');
                    return true;
                }
                return false;
            }
            _ => return false,
        }
    }

    fn set_char(&mut self, x: isize, y: isize, target: char) {
        self.grid[x as usize][y as usize] = target;
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
