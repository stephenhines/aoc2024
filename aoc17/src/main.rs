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

#[derive(Debug)]
#[repr(u8)]
enum OPCODE {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for OPCODE {
    fn from(v: u8) -> OPCODE {
        match v {
            0 => OPCODE::Adv,
            1 => OPCODE::Bxl,
            2 => OPCODE::Bst,
            3 => OPCODE::Jnz,
            4 => OPCODE::Bxc,
            5 => OPCODE::Out,
            6 => OPCODE::Bdv,
            7 => OPCODE::Cdv,
            _ => { panic!("Unhandled Opcode {}", v); },
        }
    }
}

#[derive(Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(lines: &Vec<String>) -> Self {
        let mut register_a = 0;
        let mut register_b = 0;
        let mut register_c = 0;
        let ip = 0;
        let mut program = Vec::new();
        let output = Vec::new();
        for line in lines {
            let toks: Vec<&str> = line.split_ascii_whitespace().collect();
            if toks.is_empty() { continue; }
            match toks[0] {
                "Register" => {
                    match toks[1] {
                        "A:" => { register_a = toks[2].parse::<u64>().unwrap(); },
                        "B:" => { register_b = toks[2].parse::<u64>().unwrap(); },
                        "C:" => { register_c = toks[2].parse::<u64>().unwrap(); },
                        _ => { panic!("Unhandled register: {}", line); },
                    }
                },
                "Program:" => {
                    toks[1].split(',').for_each(|d| program.push(d.parse::<u8>().unwrap()));
                },
                _ => { panic!("Unknown line: {}", line); }
            }
        }
        Computer { register_a, register_b, register_c, ip, program, output }
    }

    fn read_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => { operand as u64 },
            4 => { self.register_a },
            5 => { self.register_b },
            6 => { self.register_c },
            _ => { panic!("Invalid combo operand: {}", operand); }
        }
    }

    fn get_opcode(&self) -> OPCODE {
        self.program[self.ip].into()
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }
        let opcode = self.get_opcode();
        match opcode {
            OPCODE::Adv | OPCODE::Bdv | OPCODE::Cdv => {
                // Rx = Rx / 2^(combo_operand)
                let numerator = self.register_a;
                let operand = self.program[self.ip + 1];
                let combo_operand = self.read_combo_operand(operand) as u32;
                let denominator = 2u64.pow(combo_operand);
                let div = numerator / denominator;
                match opcode {
                    OPCODE::Adv => { self.register_a = div; },
                    OPCODE::Bdv => { self.register_b = div; },
                    OPCODE::Cdv => { self.register_c = div; },
                    _ => { unreachable!(); }
                }
                self.ip += 2;
            },
            OPCODE::Bxl => {
                // B = bitwise XOR of literal operand with register B
                let literal_operand = self.program[self.ip + 1] as u64;
                self.register_b ^= literal_operand;
                self.ip += 2;
            },
            OPCODE::Bst => {
                // B = combo_operand % 8
                let operand = self.program[self.ip + 1];
                let combo_operand = self.read_combo_operand(operand);
                self.register_b = combo_operand % 8;
                self.ip += 2;
            },
            OPCODE::Jnz => {
                // Jump to literal_operand if A != 0
                if self.register_a == 0 {
                    self.ip += 2;
                } else {
                    let literal_operand = self.program[self.ip + 1];
                    self.ip = literal_operand as usize;
                }
            },
            OPCODE::Bxc => {
                // B = B ^ C
                self.register_b ^= self.register_c;
                self.ip += 2;
            },
            OPCODE::Out => {
                // Output combo_operand % 8
                let operand = self.program[self.ip + 1];
                let combo_operand = self.read_combo_operand(operand);
                self.output.push((combo_operand % 8) as u8);
                self.ip += 2;
            }
        }
        true
    }

    fn run_program(&mut self) -> String {
        while self.step() {};
        let mut result = String::new();
        let mut first = true;
        for o in &self.output {
            if !first {
                result.push(',');
            } else {
                first = false;
            }
            result.push_str(o.to_string().as_str());
        }
        result
    }
} 

#[test]
fn test_prelim() {
    let mut computer = Computer::new(&get_input("prelim.txt"));
    let output = computer.run_program();
    assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn test_part1() {
    let mut computer = Computer::new(&get_input("input.txt"));
    let output = computer.run_program();
    assert_eq!(output, "3,6,3,7,0,7,0,3,0");
}

#[test]
fn test_combo_operand() {
    let mut computer = Computer::new(&get_input("prelim.txt"));
    assert_eq!(computer.read_combo_operand(0), 0);
    assert_eq!(computer.read_combo_operand(1), 1);
    assert_eq!(computer.read_combo_operand(2), 2);
    assert_eq!(computer.read_combo_operand(3), 3);
    computer.register_a = 5;
    assert_eq!(computer.read_combo_operand(4), 5);
    computer.register_b = 6;
    assert_eq!(computer.read_combo_operand(5), 6);
    computer.register_c = 7;
    assert_eq!(computer.read_combo_operand(6), 7);
}

fn main() {
    let mut computer = Computer::new(&get_input("prelim.txt"));
    let output = computer.run_program();
    println!("output: {}", output);
    let mut computer = Computer::new(&get_input("input.txt"));
    let output = computer.run_program();
    println!("output: {}", output);
}
