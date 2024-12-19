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

fn create_from_3bit(val: &Vec<u8>) -> u64 {
    let mut result = 0;
    for v in val {
        result <<= 3;
        result |= *v as u64;
    }
    result
}

#[derive(Debug)]
#[repr(u8)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Opcode {
        match v {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => {
                panic!("Unhandled Opcode {}", v);
            }
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
            if toks.is_empty() {
                continue;
            }
            match toks[0] {
                "Register" => match toks[1] {
                    "A:" => {
                        register_a = toks[2].parse::<u64>().unwrap();
                    }
                    "B:" => {
                        register_b = toks[2].parse::<u64>().unwrap();
                    }
                    "C:" => {
                        register_c = toks[2].parse::<u64>().unwrap();
                    }
                    _ => {
                        panic!("Unhandled register: {}", line);
                    }
                },
                "Program:" => {
                    toks[1]
                        .split(',')
                        .for_each(|d| program.push(d.parse::<u8>().unwrap()));
                }
                _ => {
                    panic!("Unknown line: {}", line);
                }
            }
        }
        Computer {
            register_a,
            register_b,
            register_c,
            ip,
            program,
            output,
        }
    }

    fn read_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => {
                panic!("Invalid combo operand: {}", operand);
            }
        }
    }

    fn get_opcode(&self) -> Opcode {
        self.program[self.ip].into()
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }
        let opcode = self.get_opcode();
        match opcode {
            Opcode::Adv | Opcode::Bdv | Opcode::Cdv => {
                // Rx = Ra / 2^(combo_operand)
                //   OR
                // Rx = Ra >> combo_operand
                let operand = self.program[self.ip + 1];
                let combo_operand = self.read_combo_operand(operand) as u32;
                let shift = self.register_a >> combo_operand;
                match opcode {
                    Opcode::Adv => {
                        self.register_a = shift;
                    }
                    Opcode::Bdv => {
                        self.register_b = shift;
                    }
                    Opcode::Cdv => {
                        self.register_c = shift;
                    }
                    _ => {
                        unreachable!();
                    }
                }
                self.ip += 2;
            }
            Opcode::Bxl => {
                // B = bitwise XOR of literal operand with register B
                let literal_operand = self.program[self.ip + 1] as u64;
                self.register_b ^= literal_operand;
                self.ip += 2;
            }
            Opcode::Bst => {
                // B = combo_operand % 8
                let operand = self.program[self.ip + 1];
                let combo_operand = self.read_combo_operand(operand);
                self.register_b = combo_operand % 8;
                self.ip += 2;
            }
            Opcode::Jnz => {
                // Jump to literal_operand if A != 0
                if self.register_a == 0 {
                    self.ip += 2;
                } else {
                    let literal_operand = self.program[self.ip + 1];
                    self.ip = literal_operand as usize;
                }
            }
            Opcode::Bxc => {
                // B = B ^ C
                self.register_b ^= self.register_c;
                self.ip += 2;
            }
            Opcode::Out => {
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
        while self.step() {}
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

    fn reset(&mut self, start_a: u64, start_b: u64, start_c: u64) {
        self.register_a = start_a;
        self.register_b = start_b;
        self.register_c = start_c;
        self.ip = 0;
        self.output.clear();
    }

    fn find_quine(&mut self) -> u64 {
        // Our register_a value really needs to be the same length as the program
        let prog_len = self.program.len();
        let mut try_a_vec = vec![0; prog_len];
        let start_b = self.register_b;
        let start_c = self.register_c;

        // test_index is where we start from in our vector of register_a values to try.
        // We move from most significant to least significant 3-bit groups as we search
        // for the correct answer.
        let mut test_index = 0;
        loop {
            let start_a = create_from_3bit(&try_a_vec);
            self.reset(start_a, start_b, start_c);
            self.run_program();

            // Success!
            if self.output == self.program {
                return start_a;
            }

            // We only need to examine the least significant 3-bit groupings from the program to
            // determine if we're still on a valid path towards a solution. The prog_len check is
            // necessary because some attempted register_a values might not produce enough 3-bit
            // groupings at all to look for the "last" few trios.
            let mirror_index = prog_len - test_index - 1;
            if self.output.len() == prog_len
                && self.output[mirror_index] == self.program[mirror_index]
            {
                // Keep advancing forward in this case. We continue because we might have gotten
                // multiple digits correct already, but this makes for simpler logic to not start
                // jumping too far ahead.
                test_index += 1;
                continue;
            }

            // If we're still not correct for this particular bit trio, we need to increment.
            // Unfortunately, if we are at the end of our 3-bit trio values, we need to reset
            // all subsequent trios back to 0, and then go back an index and potentially
            // increment that location. Of course, we can have multiple values of 7 in a row,
            // which results in continuing to apply this strategy.
            loop {
                let test_val = try_a_vec[test_index];
                if test_val == 7 {
                    // If we run out of bits in our trio, we need to go back in indices.
                    // Reset subsequent values.
                    try_a_vec.iter_mut().skip(test_index).for_each(|x| {
                        *x = 0;
                    });
                    test_index -= 1;
                } else {
                    try_a_vec[test_index] += 1;
                    break;
                }
            }
        }
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
fn test_prelim2() {
    let mut computer = Computer::new(&get_input("prelim2.txt"));
    let start_a = computer.find_quine();
    assert_eq!(start_a, 117440);
}

#[test]
fn test_part2() {
    let mut computer = Computer::new(&get_input("input.txt"));
    let start_a = computer.find_quine();
    assert_eq!(start_a, 136904920099226);
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
    let mut computer = Computer::new(&get_input("input.txt"));
    let start_a = computer.find_quine();
    println!("start_a: {}", start_a);
}
