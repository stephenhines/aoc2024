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
enum Operation {
    Add,
    Mul,
}

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

fn read_equations(lines: &[String]) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in lines {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        let result = lhs.parse::<usize>().unwrap();
        let operands = rhs.split_ascii_whitespace().map(|o| o.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let eq = Equation {result, operands};
        equations.push(eq);
    }

    equations
}

// target - the amount we want to compute
// result - our current running total
// operands - the immutable list of operands
// operation_index - where we currently are working
// operation - the add/mul we should apply
fn try_operation(target: usize, result: usize, operands: &Vec<usize>, operation_index: usize, operation: Operation) -> bool {
    if result > target {
        return false;
    }

    // Perform the operation
    let new_result = match operation {
        Operation::Add => {
            result + operands[operation_index + 1]
        },
        Operation::Mul => {
            result * operands[operation_index + 1]
        },
    };

    if operation_index + 2 == operands.len() {
        return new_result == target;
    } else {
        // Recurse by checking add and/or multiply, but early exit if we're already over
        if new_result > target {
            return false;
        }

        // Try add and then try multiply recursively
        if try_operation(target, new_result, operands, operation_index + 1, Operation::Add) {
            return true;
        }
        if try_operation(target, new_result, operands, operation_index + 1, Operation::Mul) {
            return true;
        }
    }
    false
}

fn calibrate(equation: &Equation) -> bool {
    let target = equation.result;
    let operands = &equation.operands;
    let result = operands[0];
    if try_operation(target, result, operands, 0, Operation::Add) {
        return true;
    }
    if try_operation(target, result, operands, 0, Operation::Mul) {
        return true;
    }
    false
}

fn calibrate_all(equations: &Vec<Equation>) -> usize {
    let mut sum = 0;
    for equation in equations {
        if calibrate(equation) {
            sum += equation.result;
        }
    }

    println!("Calibration sum {sum}");
    sum
}

#[test]
fn test_prelim() {
    let sum = calibrate_all(&read_equations(&get_input("prelim.txt")));
    assert_eq!(sum, 3749);
}

#[test]
fn test_part1() {
    let sum = calibrate_all(&read_equations(&get_input("input.txt")));
    assert_eq!(sum, 20281182715321);
}

fn main() {
    calibrate_all(&read_equations(&get_input("prelim.txt")));
    calibrate_all(&read_equations(&get_input("input.txt")));
}
