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
    fn check<'a>(&self, design: &'a [char], lookup: &mut HashMap<&'a [char], bool>) -> bool {
        //println!("Checking {:?}", design);
        if design.is_empty() {
            return true;
        }

        if lookup.contains_key(design) {
            return *lookup.get(design).unwrap();
        }

        for pattern in &self.patterns {
            if design.starts_with(pattern) && self.check(&design[pattern.len()..], lookup) {
                lookup.insert(design, true);
                return true;
            }
        }

        // We didn't find a viable pattern this time
        lookup.insert(design, false);
        false
    }

    fn check_designs(&self) -> usize {
        let mut valid = 0;
        let mut lookup: HashMap<&[char], bool> = HashMap::new();

        for design in &self.designs {
            if self.check(design, &mut lookup) {
                valid += 1;
            }
        }

        println!("Valid: {valid}");
        valid
    }
}

#[test]
fn test_prelim() {
    let valid = Onsen::new(&get_input("prelim.txt")).check_designs();
    assert_eq!(valid, 6);
}

#[test]
fn test_part1() {
    let valid = Onsen::new(&get_input("input.txt")).check_designs();
    assert_eq!(valid, 327);
}

fn main() {
    Onsen::new(&get_input("prelim.txt")).check_designs();
    Onsen::new(&get_input("input.txt")).check_designs();
}
