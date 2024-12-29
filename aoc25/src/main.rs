use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

const WIDTH: usize = 5;
const HEIGHT: usize = 7;

type Heights = [i32; WIDTH];

#[derive(Debug)]
struct KeyOrLock {
    heights: Heights,
}

impl KeyOrLock {
    fn new(heights: Heights) -> Self {
        KeyOrLock { heights }
    }
}

#[derive(Debug)]
struct Tumblers {
    keys: Vec<KeyOrLock>,
    locks: Vec<KeyOrLock>,
}

impl Tumblers {
    fn new() -> Self {
        Tumblers {
            keys: Vec::new(),
            locks: Vec::new(),
        }
    }

    fn num_fit(&self) -> u64 {
        let mut fits = 0;

        for key in &self.keys {
            for lock in &self.locks {
                let valid = key
                    .heights
                    .iter()
                    .zip(lock.heights.iter())
                    .fold(true, |v, (kh, lh)| v && kh + lh <= WIDTH as i32);
                if valid {
                    fits += 1;
                }
            }
        }
        println!("Fits: {fits}");
        fits
    }
}

#[derive(PartialEq)]
enum Type {
    Key,
    Lock,
    Unknown,
}

fn read_tumblers(lines: &Vec<String>) -> Tumblers {
    let mut tumblers = Tumblers::new();
    let mut cur_block: Heights = [-1; WIDTH];
    let mut cur_type = Type::Unknown;
    let mut cur_height = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if cur_height == 0 && cur_type == Type::Unknown && line == "#####" {
            // Blocks starting with ##### indicate a lock.
            cur_type = Type::Lock;
        } else if cur_height == 0 && cur_type == Type::Unknown && line == "....." {
            // Blocks starting with ..... indicate a key.
            cur_type = Type::Key;
        }

        for (i, c) in line.char_indices().collect::<Vec<_>>() {
            if c == '#' {
                cur_block[i] += 1;
            }
        }

        cur_height += 1;
        if cur_height == HEIGHT {
            match cur_type {
                Type::Key => {
                    let new_key = KeyOrLock::new(cur_block);
                    tumblers.keys.push(new_key);
                }
                Type::Lock => {
                    let new_lock = KeyOrLock::new(cur_block);
                    tumblers.locks.push(new_lock);
                }
                Type::Unknown => {
                    panic!("Unknown Key/Lock type");
                }
            }

            // Reset everything
            cur_type = Type::Unknown;
            cur_height = 0;
            cur_block = [-1; WIDTH];
        }
    }
    tumblers
}

#[test]
fn test_prelim() {
    let fit = read_tumblers(&get_input("prelim.txt")).num_fit();
    assert_eq!(fit, 3);
}

#[test]
fn test_part1() {
    let fit = read_tumblers(&get_input("input.txt")).num_fit();
    assert_eq!(fit, 2993);
}

fn main() {
    read_tumblers(&get_input("prelim.txt")).num_fit();
    read_tumblers(&get_input("input.txt")).num_fit();
}
