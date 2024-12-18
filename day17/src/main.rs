use std::{fs, time::Instant};

struct Puzzle {
    program: Vec<i64>,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let input = input.lines().collect::<Vec<_>>();
        let reg_a = input[0].split_once(": ").unwrap().1.parse().unwrap();
        let reg_b = input[1].split_once(": ").unwrap().1.parse().unwrap();
        let reg_c = input[2].split_once(": ").unwrap().1.parse().unwrap();
        let program = input[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        Self {
            program,
            reg_a,
            reg_b,
            reg_c,
        }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> String {
        let output = self.run_program(self.reg_a);

        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part_2(&mut self) -> isize {
        // Find the value for the condition
        let mut answers = Vec::new();
        let mut todo = vec![(1, 0)];
        while let Some((i, a)) = todo.pop() {
            for a_val in a..a + 8 {
                if self.run_program(a_val)[..] == self.program[self.program.len() - i..] {
                    todo.push((i + 1, a_val * 8));
                    if i == self.program.len() {
                        answers.push(a_val);
                    }
                }
            }
        }

        answers.into_iter().min().unwrap() as isize
    }

    fn run_program(&mut self, reg_a: i64) -> Vec<i64> {
        let mut instr_pointer = 0;
        let mut output = Vec::new();
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;

        while instr_pointer < self.program.len() {
            match self.program[instr_pointer] {
                0 => {
                    self.reg_a = self.divide(self.program[instr_pointer + 1]);
                    instr_pointer += 2;
                }
                1 => {
                    self.reg_b ^= self.program[instr_pointer + 1] as i64;
                    instr_pointer += 2;
                }
                2 => {
                    self.reg_b = self.combo_operand(self.program[instr_pointer + 1]) % 8;
                    instr_pointer += 2;
                }
                3 => {
                    if self.reg_a != 0 {
                        instr_pointer = self.program[instr_pointer + 1] as usize;
                    } else {
                        instr_pointer += 2;
                    }
                }
                4 => {
                    self.reg_b ^= self.reg_c;
                    instr_pointer += 2;
                }
                5 => {
                    output.push(self.combo_operand(self.program[instr_pointer + 1]) % 8);
                    instr_pointer += 2;
                }
                6 => {
                    self.reg_b = self.divide(self.program[instr_pointer + 1]);
                    instr_pointer += 2;
                }
                7 => {
                    self.reg_c = self.divide(self.program[instr_pointer + 1]);
                    instr_pointer += 2;
                }
                _ => panic!("Invalid operator input!"),
            }
        }
        output
    }

    fn combo_operand(&self, op: i64) -> i64 {
        match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo input!"),
        }
    }

    fn divide(&self, op: i64) -> i64 {
        let c = self.combo_operand(op);
        let bottom = 2_i64.pow(c as u32);
        let vvv = self.reg_a as f64 / bottom as f64;
        return vvv.floor() as i64;
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
