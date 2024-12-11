use std::{collections::VecDeque, fs};

#[derive(Debug)]
enum DiskType {
    Empty(isize),
    File(isize, isize),
}

struct Puzzle {
    disk: VecDeque<DiskType>,
    files: Vec<(isize, isize, isize)>,
    empty: Vec<(isize, isize)>,
}

impl From<&str> for Puzzle {
    fn from(file_path: &str) -> Self {
        let input = fs::read_to_string(file_path).expect("Failed to read the string");
        let mut disk = VecDeque::<DiskType>::new();
        let mut files = Vec::<(isize, isize, isize)>::new();
        let mut empty = Vec::<(isize, isize)>::new();
        let mut file_id = 0;
        let mut is_file = true;
        let mut mem_loc = 0;

        for c in input.trim().chars() {
            let file_size = c.to_digit(10).unwrap() as isize;
            if is_file {
                disk.push_back(DiskType::File(file_id, file_size));
                files.push((file_id, mem_loc, file_size));
                mem_loc += file_size;
                file_id += 1;
                is_file = false;
            } else {
                disk.push_back(DiskType::Empty(file_size));
                empty.push((mem_loc, file_size));
                mem_loc += file_size;
                is_file = true;
            }
        }

        Self { disk, files, empty }
    }
}

impl Puzzle {
    fn part_1(&mut self) -> isize {
        let mut checksum = 0;
        let mut mem_location = 0;

        while !self.disk.is_empty() {
            let (file_id, file_size) = match self.disk.pop_front().unwrap() {
                DiskType::File(file_id, file_size) => (file_id, file_size),
                _ => panic!("Unexpected input"),
            };
            let new_mem_location = mem_location + file_size;
            checksum +=
                (new_mem_location * (new_mem_location - 1) - mem_location * (mem_location - 1)) / 2
                    * file_id;
            mem_location = new_mem_location;

            let mut free_size = match self.disk.pop_front() {
                None => break,
                Some(DiskType::Empty(emtry_size)) => emtry_size,
                _ => panic!("Unexpected input"),
            };

            while free_size > 0 {
                let (file_id, file_size) = match self.disk.back_mut() {
                    None => break,
                    Some(DiskType::File(file_id, file_size)) => (file_id, file_size),
                    _ => panic!("Unexpected input"),
                };
                if *file_size > free_size {
                    let new_mem_location = mem_location + free_size;
                    checksum += (new_mem_location * (new_mem_location - 1)
                        - mem_location * (mem_location - 1))
                        / 2
                        * *file_id;
                    mem_location = new_mem_location;
                    *file_size -= free_size;
                    free_size = 0;
                } else {
                    let new_mem_location = mem_location + *file_size;
                    checksum += (new_mem_location * (new_mem_location - 1)
                        - mem_location * (mem_location - 1))
                        / 2
                        * *file_id;
                    mem_location = new_mem_location;
                    free_size -= *file_size;
                    self.disk.pop_back();
                    match self.disk.pop_back() {
                        None => break,
                        Some(DiskType::File(_, _)) => panic!("Unexpected input"),
                        _ => (),
                    }
                }
            }
        }

        checksum
    }

    fn part_2(&mut self) -> isize {
        let mut checksum = 0;

        'file_loop: for (file_number, file_addr, file_size) in self.files.iter().rev() {
            for i in 0..self.empty.len() {
                let (space_addr, space_size) = self.empty.get_mut(i).unwrap();
                if *space_addr > *file_addr {
                    break;
                } else if *file_size == *space_size {
                    checksum += ((*space_addr + *file_size) * (*space_addr + *file_size - 1)
                        - *space_addr * (*space_addr - 1))
                        / 2
                        * *file_number;
                    self.empty.remove(i);
                    continue 'file_loop;
                } else if *file_size < *space_size {
                    checksum += ((*space_addr + *file_size) * (*space_addr + *file_size - 1)
                        - *space_addr * (*space_addr - 1))
                        / 2
                        * *file_number;
                    *space_addr += *file_size;
                    *space_size -= *file_size;
                    continue 'file_loop;
                }
            }
            checksum += ((*file_addr + *file_size) * (*file_addr + *file_size - 1)
                - file_addr * (file_addr - 1))
                / 2
                * file_number;
        }

        checksum
    }
}

fn main() {
    println!("Solution part 1: {}", Puzzle::from("input.txt").part_1());
    println!("Solution part 2: {}", Puzzle::from("input.txt").part_2());
}
