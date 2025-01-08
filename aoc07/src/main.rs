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
    Cat,
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
        let operands = rhs
            .split_ascii_whitespace()
            .map(|o| o.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let eq = Equation { result, operands };
        equations.push(eq);
    }

    equations
}

// target - the amount we want to compute
// result - our current running total
// operands - the immutable list of operands
// operation_index - where we currently are working
// operation - the add/mul we should apply
fn try_operation(
    target: usize,
    result: usize,
    operands: &Vec<usize>,
    operation_index: usize,
    operation: Operation,
    cat_supported: bool,
) -> bool {
    if result > target || (!cat_supported && operation == Operation::Cat) {
        return false;
    }

    // Perform the operation
    let new_result = match operation {
        Operation::Add => result + operands[operation_index + 1],
        Operation::Mul => result * operands[operation_index + 1],
        Operation::Cat => {
            // Just convert with a format string and count the digits to multiply by 10
            let rhs = operands[operation_index + 1];
            let num_digits = format!("{}", rhs);
            let mut result = result;
            (0..num_digits.len()).for_each(|_| result *= 10);
            result + rhs
        }
    };

    if operation_index + 2 == operands.len() {
        return new_result == target;
    } else {
        // Recurse by checking add and/or multiply, but early exit if we're already over
        if new_result > target {
            return false;
        }

        // Try add and then try multiply recursively
        if try_operation(
            target,
            new_result,
            operands,
            operation_index + 1,
            Operation::Add,
            cat_supported,
        ) {
            return true;
        }
        if try_operation(
            target,
            new_result,
            operands,
            operation_index + 1,
            Operation::Mul,
            cat_supported,
        ) {
            return true;
        }
        if cat_supported
            && try_operation(
                target,
                new_result,
                operands,
                operation_index + 1,
                Operation::Cat,
                cat_supported,
            )
        {
            return true;
        }
    }
    false
}

fn calibrate(equation: &Equation, cat_supported: bool) -> bool {
    let target = equation.result;
    let operands = &equation.operands;
    let result = operands[0];
    if try_operation(target, result, operands, 0, Operation::Add, cat_supported) {
        return true;
    }
    if try_operation(target, result, operands, 0, Operation::Mul, cat_supported) {
        return true;
    }
    if cat_supported && try_operation(target, result, operands, 0, Operation::Cat, cat_supported) {
        return true;
    }
    false
}

fn calibrate_all(equations: &Vec<Equation>, cat_supported: bool) -> usize {
    let mut sum = 0;
    for equation in equations {
        if calibrate(equation, cat_supported) {
            sum += equation.result;
        }
    }

    println!("Calibration sum {sum}");
    sum
}

#[test]
fn test_prelim() {
    let sum = calibrate_all(&read_equations(&get_input("prelim.txt")), false);
    assert_eq!(sum, 3749);
}

#[test]
fn test_part1() {
    let sum = calibrate_all(&read_equations(&get_input("input.txt")), false);
    assert_eq!(sum, 20281182715321);
}

#[test]
fn test_prelim2() {
    let sum = calibrate_all(&read_equations(&get_input("prelim.txt")), true);
    assert_eq!(sum, 11387);
}

#[test]
fn test_part2() {
    let sum = calibrate_all(&read_equations(&get_input("input.txt")), true);
    assert_eq!(sum, 159490400628354);
}

fn main() {
    calibrate_all(&read_equations(&get_input("prelim.txt")), false);
    calibrate_all(&read_equations(&get_input("input.txt")), false);
    calibrate_all(&read_equations(&get_input("prelim.txt")), true);
    calibrate_all(&read_equations(&get_input("input.txt")), true);
}
