use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn parse_muls(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    for line in lines {
        for (_, [lhs, rhs]) in re.captures_iter(line).map(|c| c.extract()) {
            let l = lhs.parse::<u32>().unwrap();
            let r = rhs.parse::<u32>().unwrap();
            sum += l * r;
        }
    }
    println!("Sum: {}", sum);
    sum
}

#[test]
fn test_prelim() {
    let sum = parse_muls(&get_input("prelim.txt"));
    assert_eq!(sum, 161);
}

#[test]
fn test_part1() {
    let sum = parse_muls(&get_input("input.txt"));
    assert_eq!(sum, 188116424);
}

fn main() {
    parse_muls(&get_input("prelim.txt"));
    parse_muls(&get_input("input.txt"));
}
