use std::collections::HashMap;
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

struct Pluto {
    stones: Vec<usize>,
}

impl Pluto {
    fn new(lines: &[String]) -> Self {
        let mut stones = Vec::new();
        assert_eq!(lines.len(), 1);
        let toks = lines[0].split_ascii_whitespace().collect::<Vec<_>>();
        for tok in toks {
            let val = tok.parse::<usize>().unwrap();
            stones.push(val);
        }

        Self { stones }
    }

    fn blink_r(stone: usize, n: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
        if memo.contains_key(&(stone, n)) {
            return *memo.get(&(stone, n)).unwrap();
        }

        let mut total = 0;
        if n == 0 {
            // Recursive base case just returns 1 for the stone that we are
            total = 1;
        } else if stone == 0 {
            // Rule 1: Replace 0 with 1
            total += Self::blink_r(1, n - 1, memo);
        } else if format!("{stone}").len() % 2 == 0 {
            // Rule 2: Split even number of digits into top and bottom halves
            let stone_str = format!("{stone}");
            let top = stone_str[..stone_str.len() / 2].parse::<usize>().unwrap();
            let bot = stone_str[stone_str.len() / 2..].parse::<usize>().unwrap();
            total += Self::blink_r(top, n - 1, memo);
            total += Self::blink_r(bot, n - 1, memo);
        } else {
            // Rule 3: Replace anything else with the number multiplied by 2024
            total += Self::blink_r(stone * 2024, n - 1, memo);
        }
        memo.insert((stone, n), total);

        total
    }

    // Returns the number of stones after blinking n times
    fn blink(&self, n: usize) -> usize {
        let mut total = 0;
        let mut memo = HashMap::new();

        for stone in &self.stones {
            total += Self::blink_r(*stone, n, &mut memo);
        }

        println!("Blinks({n}): {total}");
        total
    }
}

#[test]
fn test_prelim() {
    let count = Pluto::new(&get_input("prelim.txt")).blink(6);
    assert_eq!(count, 22);

    let count = Pluto::new(&get_input("prelim.txt")).blink(25);
    assert_eq!(count, 55312);
}

#[test]
fn test_part1() {
    let count = Pluto::new(&get_input("input.txt")).blink(25);
    assert_eq!(count, 224529);
}

#[test]
fn test_part2() {
    let count = Pluto::new(&get_input("input.txt")).blink(75);
    assert_eq!(count, 266820198587914);
}

fn main() {
    Pluto::new(&get_input("prelim.txt")).blink(6);
    Pluto::new(&get_input("prelim.txt")).blink(25);
    Pluto::new(&get_input("input.txt")).blink(25);
    Pluto::new(&get_input("input.txt")).blink(75);
}
