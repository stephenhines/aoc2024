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

type LookupType<'a> = HashMap<&'a [char], usize>;

struct Onsen {
    patterns: Vec<Vec<char>>,
    designs: Vec<Vec<char>>,
}

impl Onsen {
    fn new(lines: &[String]) -> Self {
        let mut patterns = Vec::new();
        let mut designs = Vec::new();

        lines[0]
            .split(", ")
            .collect::<Vec<_>>()
            .iter()
            .for_each(|p| patterns.push(p.chars().collect::<Vec<_>>()));
        assert!(lines[1].is_empty());
        for line in &lines[2..] {
            designs.push(line.chars().collect::<Vec<_>>());
        }
        Onsen { patterns, designs }
    }

    // lookup is how we memoize our already seen results
    fn check<'a>(&self, design: &'a [char], lookup: &mut LookupType<'a>) -> usize {
        //println!("Checking {:?}", design);
        if design.is_empty() {
            return 1;
        }

        if lookup.contains_key(design) {
            return *lookup.get(design).unwrap();
        }

        let mut valid = 0;
        for pattern in &self.patterns {
            if design.starts_with(pattern) {
                valid += self.check(&design[pattern.len()..], lookup);
            }
        }

        lookup.insert(design, valid);
        valid
    }

    fn check_designs(&self, all: bool) -> usize {
        let mut valid = 0;
        let mut lookup: LookupType = HashMap::new();

        for design in &self.designs {
            let v = self.check(design, &mut lookup);
            if all {
                valid += v;
            } else if v > 0 {
                valid += 1;
            }
        }

        println!("Valid: {valid}");
        valid
    }
}

#[test]
fn test_prelim() {
    let valid = Onsen::new(&get_input("prelim.txt")).check_designs(false);
    assert_eq!(valid, 6);
}

#[test]
fn test_part1() {
    let valid = Onsen::new(&get_input("input.txt")).check_designs(false);
    assert_eq!(valid, 327);
}

#[test]
fn test_prelim2() {
    let valid = Onsen::new(&get_input("prelim.txt")).check_designs(true);
    assert_eq!(valid, 16);
}

#[test]
fn test_part2() {
    let valid = Onsen::new(&get_input("input.txt")).check_designs(true);
    assert_eq!(valid, 772696486795255);
}

fn main() {
    Onsen::new(&get_input("prelim.txt")).check_designs(false);
    Onsen::new(&get_input("input.txt")).check_designs(false);
    Onsen::new(&get_input("prelim.txt")).check_designs(true);
    Onsen::new(&get_input("input.txt")).check_designs(true);
}
