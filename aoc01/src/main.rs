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

fn parse_lines(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut v1: Vec<u32> = Vec::new();
    let mut v2: Vec<u32> = Vec::new();
    for line in lines {
        let l: Vec<&str> = line.split_whitespace().collect();
        v1.push(l[0].parse::<u32>().unwrap());
        v2.push(l[1].parse::<u32>().unwrap());
    }
    (v1, v2)
}

fn compute_sorted_diff(vs: &mut (Vec<u32>, Vec<u32>)) -> u32 {
    let mut diff: u32 = 0;
    let v1 = &mut vs.0;
    let v2 = &mut vs.1;
    v1.sort();
    v2.sort();
    //println!("v1: {:?}", v1);
    //println!("v2: {:?}", v2);

    assert!(v1.len() == v2.len());
    let num_lines = v1.len();
    for i in 0..num_lines {
        diff += u32::abs_diff(v1[i], v2[i]);
    }
    println!("diff: {}", diff);
    diff
}

#[test]
fn test_prelim() {
    let diff = compute_sorted_diff(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(diff, 11);
}

#[test]
fn test_part1() {
    let diff = compute_sorted_diff(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(diff, 1222801);
}

fn main() {
    compute_sorted_diff(&mut parse_lines(&get_input("prelim.txt")));
    compute_sorted_diff(&mut parse_lines(&get_input("input.txt")));
}
