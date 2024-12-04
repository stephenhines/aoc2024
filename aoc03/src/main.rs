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

fn parse_muls_part2(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    let mut enabled = true;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let mut i = 0;
        while i < len {
            match chars[i] {
                'd' => {
                    // Look at i + 3 since we minimally need to handle at least "do()"
                    if i + 3 < len
                        && chars[i + 1] == 'o'
                        && chars[i + 2] == '('
                        && chars[i + 3] == ')'
                    {
                        enabled = true;
                        i += 4;
                        continue;
                    } else if i + 6 < len
                        && chars[i + 1] == 'o'
                        && chars[i + 2] == 'n'
                        && chars[i + 3] == '\''
                        && chars[i + 4] == 't'
                        && chars[i + 5] == '('
                        && chars[i + 6] == ')'
                    {
                        enabled = false;
                        i += 7;
                        continue;
                    }
                }
                'm' => {
                    // Look at i + 7 since we minimally need to handle at least "mul(1,1)"
                    if enabled && i + 7 < len {
                        if chars[i + 1] == 'u' && chars[i + 2] == 'l' && chars[i + 3] == '(' {
                            // Time to parse some integers
                            i += 4;
                            if let Some((lhs, new_pos)) = parse_up_to_3_digits(&chars, i) {
                                i = new_pos;
                                // Still need to parse at least ",1)"
                                if i + 3 < len && chars[i] == ',' {
                                    i += 1;
                                    if let Some((rhs, new_pos)) = parse_up_to_3_digits(&chars, i) {
                                        i = new_pos;
                                        // Check for trailing parentheses before finishing
                                        if i < len && chars[i] == ')' {
                                            sum += lhs * rhs;
                                            i += 1;  // These seem redundant, but I prefer being clear that this parsed
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            // If we get here, we're done with the current token and just start at the next one
            i += 1;
        }
    }
    println!("Sum (do/don't): {}", sum);
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

#[test]
fn test_prelim2() {
    let sum = parse_muls_part2(&get_input("prelim2.txt"));
    assert_eq!(sum, 48);
}

#[test]
fn test_part2() {
    let sum = parse_muls_part2(&get_input("input.txt"));
    assert_eq!(sum, 104245808);
}

fn main() {
    parse_muls(&get_input("prelim.txt"));
    parse_muls(&get_input("input.txt"));
    parse_muls_part2(&get_input("prelim2.txt"));
    parse_muls_part2(&get_input("input.txt"));
}
