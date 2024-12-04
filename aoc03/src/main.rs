use regex::Regex;
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

#[derive(PartialEq)]
enum CheckEnabled {
    Checked,
    Unchecked,
}

// This was my first solution to part 1, which works nicely, but I needed a better parser for part 2
fn sum_of_multiplies_regex(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    for line in lines {
        for (_, [lhs, rhs]) in re.captures_iter(line).map(|c| c.extract()) {
            let l = lhs.parse::<u32>().unwrap();
            let r = rhs.parse::<u32>().unwrap();
            sum += l * r;
        }
    }
    println!("Sum (regex): {}", sum);
    sum
}

fn parse_up_to_3_digits(chars: &Vec<char>, pos: usize) -> Option<(u32, usize)> {
    let mut set = false;
    let mut val = 0;
    let mut new_pos = pos;
    let len = chars.len();
    // We are only allowed to accept 1-3 digits
    while new_pos < pos + 3 {
        if new_pos >= len {
            return None;
        }

        match chars[new_pos] {
            '0'..='9' => {
                val = val * 10 + (chars[new_pos] as u32 - '0' as u32);
                set = true;
                new_pos += 1;
            }
            _ => {
                if set {
                    return Some((val, new_pos));
                } else {
                    return None;
                }
            }
        }
    }
    Some((val, new_pos))
}

// Returns new cursor position if we parsed "do()"
fn parse_do(chars: &Vec<char>, pos: usize) -> Option<usize> {
    if pos + 3 < chars.len()
        && chars[pos + 0] == 'd'
        && chars[pos + 1] == 'o'
        && chars[pos + 2] == '('
        && chars[pos + 3] == ')'
    {
        Some(pos + 4)
    } else {
        None
    }
}

// Returns new cursor position if we parsed "don't()"
fn parse_dont(chars: &Vec<char>, pos: usize) -> Option<usize> {
    if pos + 6 < chars.len()
        && chars[pos + 0] == 'd'
        && chars[pos + 1] == 'o'
        && chars[pos + 2] == 'n'
        && chars[pos + 3] == '\''
        && chars[pos + 4] == 't'
        && chars[pos + 5] == '('
        && chars[pos + 6] == ')'
    {
        Some(pos + 7)
    } else {
        None
    }
}

// Returns product and new cursor position if we parsed the regex r"mul\([0-9]{1,3},[0-9]{1,3}\)"
fn parse_mul(chars: &Vec<char>, pos: usize) -> Option<(u32, usize)> {
    let len = chars.len();

    // Look at pos + 7 since we minimally need to handle at least "mul(1,1)"
    if pos + 7 >= len
        || chars[pos + 0] != 'm'
        || chars[pos + 1] != 'u'
        || chars[pos + 2] != 'l'
        || chars[pos + 3] != '('
    {
        return None;
    }

    // Time to parse some integers skipping "mul("
    let (lhs, new_pos) = parse_up_to_3_digits(&chars, pos + 4)?;

    // Still need to parse at least ",1)"
    if new_pos + 3 >= len || chars[new_pos] != ',' {
        return None;
    }

    // Skipping comma
    let (rhs, new_pos) = parse_up_to_3_digits(&chars, new_pos + 1)?;

    // Check for trailing parentheses before finishing
    if new_pos >= len || chars[new_pos] != ')' {
        return None;
    }

    let product = lhs * rhs;
    Some((product, new_pos + 1)) // Skipping rparen
}

fn sum_of_multiplies(lines: &Vec<String>, check: CheckEnabled) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut pos = 0;
        while pos < len {
            match chars[pos] {
                'd' => {
                    if let Some(new_pos) = parse_do(&chars, pos) {
                        enabled = true;
                        pos = new_pos;
                        continue;
                    } else if let Some(new_pos) = parse_dont(&chars, pos) {
                        // Only disable if we've enabled checked multiplies
                        if check == CheckEnabled::Checked {
                            enabled = false;
                        }
                        pos = new_pos;
                        continue;
                    }
                }
                'm' => {
                    if enabled {
                        if let Some((product, new_pos)) = parse_mul(&chars, pos) {
                            sum += product;
                            pos = new_pos;
                            continue;
                        }
                    }
                }
                _ => {}
            }
            // If we get here, we're done with the current token and just start at the next one
            pos += 1;
        }
    }
    let check_msg = if check == CheckEnabled::Unchecked {
        "unchecked"
    } else {
        "checked"
    };
    println!("Sum ({}): {}", check_msg, sum);
    sum
}

fn sum_of_all_multiplies(lines: &Vec<String>) -> u32 {
    sum_of_multiplies(lines, CheckEnabled::Unchecked)
}

fn sum_of_enabled_multiplies(lines: &Vec<String>) -> u32 {
    sum_of_multiplies(lines, CheckEnabled::Checked)
}

#[test]
fn test_prelim() {
    let sum = sum_of_all_multiplies(&get_input("prelim.txt"));
    assert_eq!(sum, 161);
    let sum = sum_of_multiplies_regex(&get_input("prelim.txt"));
    assert_eq!(sum, 161);
}

#[test]
fn test_part1() {
    let sum = sum_of_all_multiplies(&get_input("input.txt"));
    assert_eq!(sum, 188116424);
    let sum = sum_of_multiplies_regex(&get_input("input.txt"));
    assert_eq!(sum, 188116424);
}

#[test]
fn test_prelim2() {
    let sum = sum_of_enabled_multiplies(&get_input("prelim2.txt"));
    assert_eq!(sum, 48);
}

#[test]
fn test_part2() {
    let sum = sum_of_enabled_multiplies(&get_input("input.txt"));
    assert_eq!(sum, 104245808);
}

fn main() {
    sum_of_multiplies_regex(&get_input("prelim.txt"));
    sum_of_multiplies_regex(&get_input("input.txt"));
    sum_of_all_multiplies(&get_input("prelim.txt"));
    sum_of_all_multiplies(&get_input("input.txt"));
    sum_of_enabled_multiplies(&get_input("prelim2.txt"));
    sum_of_enabled_multiplies(&get_input("input.txt"));
}
