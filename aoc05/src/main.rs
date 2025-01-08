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

struct PageData {
    precedence_table: PrecedenceTable,
    page_list: Vec<Vec<usize>>,
}

fn read_page_info(lines: &[String]) -> PageData {
    let mut precedence_table: PrecedenceTable = [Precedence::Unknown; 100 * 100];
    let mut page_list = Vec::new();

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
            page_numbers.push(tok.parse::<usize>().unwrap());
        }
        page_list.push(page_numbers);
    }

    PageData {
        precedence_table,
        page_list,
    }
}

fn check_valid(precedence_table: &PrecedenceTable, page_numbers: &[usize]) -> bool {
    let mut seen_pages = Vec::new();
    for page in page_numbers {
        if seen_pages.contains(page) {
            panic!("Duplicate page {page}");
        }

        for before in &seen_pages {
            if precedence_table[before * 100 + page] == Precedence::After {
                return false;
            };
        }

        seen_pages.push(*page);
    }

    true
}

fn calculate_middle_page_sum(page_data: &PageData) -> usize {
    let mut sum = 0;
    let page_list = &page_data.page_list;
    let precedence_table = &page_data.precedence_table;
    for page_numbers in page_list {
        if check_valid(precedence_table, page_numbers) {
            let mid = page_numbers.len() / 2;
            sum += page_numbers[mid];
        }
    }

    println!("Total middle page sum: {sum}");
    sum
}

fn calculate_invalid_middle_page_sum(page_data: &PageData) -> usize {
    let mut sum = 0;
    let page_list = page_data.page_list.clone();
    let precedence_table = &page_data.precedence_table;
    for mut page_numbers in page_list {
        // Only consider invalid sequences
        if check_valid(precedence_table, &page_numbers) {
            continue;
        }
        for b in 0..page_numbers.len() {
            for a in b + 1..page_numbers.len() {
                let before = page_numbers[b];
                let after = page_numbers[a];
                // Simple swap if we're supposed to actually be after
                if precedence_table[before * 100 + after] == Precedence::After {
                    page_numbers[b] = after;
                    page_numbers[a] = before;
                }
            }
        }

        let valid = check_valid(precedence_table, &page_numbers);
        if valid {
            //println!("valid: {:?}", page_numbers);
            let mid = page_numbers.len() / 2;
            sum += page_numbers[mid];
        } else {
            panic!("Invalid swapped sequence {:?}", page_numbers);
        }
    }

    println!("Total invalid middle page sum: {sum}");
    sum
}

#[test]
fn test_prelim() {
    let page_data = read_page_info(&get_input("prelim.txt"));
    let sum = calculate_middle_page_sum(&page_data);
    assert_eq!(sum, 143);
}

#[test]
fn test_part1() {
    let page_data = read_page_info(&get_input("input.txt"));
    let sum = calculate_middle_page_sum(&page_data);
    assert_eq!(sum, 4578);
}

#[test]
fn test_prelim2() {
    let page_data = read_page_info(&get_input("prelim.txt"));
    let sum = calculate_invalid_middle_page_sum(&page_data);
    assert_eq!(sum, 123);
}

#[test]
fn test_part2() {
    let page_data = read_page_info(&get_input("input.txt"));
    let sum = calculate_invalid_middle_page_sum(&page_data);
    assert_eq!(sum, 6179);
}

fn main() {
    let page_data = read_page_info(&get_input("prelim.txt"));
    calculate_middle_page_sum(&page_data);
    calculate_invalid_middle_page_sum(&page_data);

    let page_data = read_page_info(&get_input("input.txt"));
    calculate_middle_page_sum(&page_data);
    calculate_invalid_middle_page_sum(&page_data);
}
