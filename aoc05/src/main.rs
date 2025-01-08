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

#[derive(Clone, Copy, PartialEq)]
enum Precedence {
    Unknown = 0,
    Before,
    After,
}

// Indexed as [BEFORE * 100 + AFTER]
type PrecedenceTable = [Precedence; 100 * 100];

fn calculate_middle_page_sum(lines: &[String]) -> usize {
    //let mut precedence_table: PrecedenceTable = [[Precedence::Unknown; 100]; 100];
    let mut precedence_table: PrecedenceTable = [Precedence::Unknown; 100 * 100];
    let mut sum = 0;
    let mut line_iter = lines.iter();
    loop {
        match line_iter.next() {
            None => {
                panic!("Invalid input");
            }
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let toks = line.split('|').collect::<Vec<_>>();
                let before = toks[0].parse::<usize>().unwrap();
                let after = toks[1].parse::<usize>().unwrap();
                assert!(before < 100);
                assert!(after < 100);
                precedence_table[before * 100 + after] = Precedence::Before;
                precedence_table[after * 100 + before] = Precedence::After;
            }
        }
    }

    // Get the remaining lines
    for line in line_iter {
        let mut page_numbers = Vec::new();
        let toks = line.split(',').collect::<Vec<_>>();
        for tok in toks {
            let val = tok.parse::<usize>().unwrap();
            page_numbers.push(val);
        }

        let mut valid = true;
        let mut seen_pages = Vec::new();
        for page in page_numbers {
            if seen_pages.contains(&page) {
                panic!("Duplicate page {page}");
            }

            for before in &seen_pages {
                if precedence_table[before * 100 + page] == Precedence::After {
                    valid = false;
                    break;
                };
            }

            if !valid {
                break;
            }

            seen_pages.push(page);
        }

        if !valid {
            continue;
        }

        //println!("valid line: {line}");
        let mid = seen_pages.len() / 2;
        sum += seen_pages[mid];
    }

    println!("Total middle page sum: {sum}");
    sum
}

#[test]
fn test_prelim() {
    let sum = calculate_middle_page_sum(&get_input("prelim.txt"));
    assert_eq!(sum, 143);
}

#[test]
fn test_part1() {
    let sum = calculate_middle_page_sum(&get_input("input.txt"));
    assert_eq!(sum, 4578);
}


fn main() {
    calculate_middle_page_sum(&get_input("prelim.txt"));
    calculate_middle_page_sum(&get_input("input.txt"));
}
