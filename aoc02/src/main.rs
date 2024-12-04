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

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut rows: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        line.split_whitespace().for_each(|x| row.push(x.parse::<u32>().unwrap()));
        rows.push(row);
    }
    //dbg!(&rows);
    rows
}

fn check_safety(row: &Vec<u32>) -> bool {
    let len = row.len();
    assert!(len >= 2);
    let mut prev = row[0];
    let up = row[0] < row[1];
    for i in 1..len {
        let cur = row[i];
        let diff = cur as i64 - prev as i64 ;
        if up {
            if diff < 1 || diff > 3 {
                return false;
            }
        } else {
            if diff > -1 || diff < -3 {
                return false;
            }
        }
        prev = cur;
    }
    true
}

fn compute_safe(rows: &Vec<Vec<u32>>) -> u32 {
    let mut safe = 0;
    for row in rows {
        if check_safety(row) == true {
            safe += 1;
        }
    }
    println!("safe: {}", safe);
    safe
}

fn compute_safe_with_problem_dampener(rows: &Vec<Vec<u32>>) -> u32 {
    let mut safe = 0;
    for row in rows {
        if check_safety(row) == true {
            safe += 1;
        } else {
            // Just clone a duplicate row and remove an item to try them
            let len = row.len();
            let mut i = 0;
            let mut done = false;
            while i < len && !done {
                let mut new_row = row.clone();
                new_row.remove(i);
                if check_safety(&new_row) == true {
                    safe += 1;
                    done = true;
                }
                i += 1;
            }
        }
    }
    println!("safe (pd): {}", safe);
    safe
}

#[test]
fn test_prelim() {
    let safe = compute_safe(&parse_lines(&get_input("prelim.txt")));
    assert_eq!(safe, 2);
}

#[test]
fn test_prelim2() {
    let safe = compute_safe_with_problem_dampener(&parse_lines(&get_input("prelim.txt")));
    assert_eq!(safe, 4);
}

#[test]
fn test_part1() {
    let safe = compute_safe(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(safe, 463);
}

#[test]
fn test_part2() {
    let safe = compute_safe_with_problem_dampener(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(safe, 514);
}

fn main() {
    compute_safe(&parse_lines(&get_input("prelim.txt")));
    compute_safe(&parse_lines(&get_input("input.txt")));
    compute_safe_with_problem_dampener(&parse_lines(&get_input("prelim.txt")));
    compute_safe_with_problem_dampener(&parse_lines(&get_input("input.txt")));
}
