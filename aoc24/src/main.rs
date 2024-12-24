use std::collections::HashMap;
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

#[derive(Clone, Debug)]
enum GateOp {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct Gate {
    gate_op: GateOp,
    operand_1: String,
    operand_2: String,
}

impl Gate {
    fn new(op: &str, operand_1: String, operand_2: String) -> Self {
        let gate_op = match op {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => panic!("Invalid op: {op}"),
        };
        Self {
            gate_op,
            operand_1,
            operand_2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WireState {
    True,
    False,
    Gated,
}

#[derive(Clone, Debug)]
struct Wire {
    name: String,
    state: WireState,
    gate: Option<Gate>,
}

impl Wire {
    fn new(line: &str) -> Self {
        if line.is_empty() {
            panic!("Invalid wire line: {line}");
        }
        if line.contains(':') {
            Self::new_init_wire(line)
        } else {
            Self::new_gate_wire(line)
        }
    }

    fn new_init_wire(line: &str) -> Self {
        // x00: 1
        let toks = line.split(':').collect::<Vec<_>>();
        assert_eq!(toks.len(), 2);
        let name = toks[0].to_string();
        let state = match toks[1].trim() {
            "1" => WireState::True,
            "0" => WireState::False,
            other => panic!("Invalid state {other} for {line}"),
        };
        Self {
            name,
            state,
            gate: None,
        }
    }

    fn new_gate_wire(line: &str) -> Self {
        // x00 AND y00 -> z00
        let toks = line.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(toks.len(), 5);
        assert_eq!(toks[3], "->");
        let operand_1 = toks[0].to_string();
        let operand_2 = toks[2].to_string();
        let name = toks[4].to_string();
        let gate = Some(Gate::new(toks[1], operand_1, operand_2));
        Self {
            name,
            state: WireState::Gated,
            gate,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Wire>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            wires: HashMap::new(),
        }
    }

    fn add_wire(&mut self, wire: &Wire) {
        self.wires.insert(wire.name.clone(), wire.clone());
    }

    fn resolve_wire(&mut self, name: &str) -> WireState {
        let wire = self.wires.get(name).unwrap().clone();
        let state = wire.state;
        match state {
            WireState::True | WireState::False => state,
            WireState::Gated => {
                let gate = wire.gate.as_ref().unwrap();
                let operand_1 = self.resolve_wire(&gate.operand_1);
                assert!(operand_1 == WireState::True || operand_1 == WireState::False);
                let operand_2 = self.resolve_wire(&gate.operand_2);
                assert!(operand_2 == WireState::True || operand_2 == WireState::False);
                let gate_op = &gate.gate_op;
                let result = match gate_op {
                    GateOp::And => {
                        if operand_1 == WireState::True && operand_2 == WireState::True {
                            WireState::True
                        } else {
                            WireState::False
                        }
                    }
                    GateOp::Or => {
                        if operand_1 == WireState::True || operand_2 == WireState::True {
                            WireState::True
                        } else {
                            WireState::False
                        }
                    }
                    GateOp::Xor => {
                        if operand_1 != operand_2 {
                            WireState::True
                        } else {
                            WireState::False
                        }
                    }
                };
                let wire = Wire {
                    name: name.to_string(),
                    state: result,
                    gate: None,
                };
                self.wires.insert(name.to_string(), wire);
                result
            }
        }
    }

    fn simulate(&mut self) -> u64 {
        let mut names = Vec::new();
        for (name, _) in self.wires.iter() {
            names.push(name.clone());
        }

        for name in names {
            self.resolve_wire(&name);
        }

        let mut zvec = self
            .wires
            .keys()
            .filter(|&n| n.starts_with('z'))
            .collect::<Vec<_>>();
        zvec.sort();
        zvec.reverse();
        let mut result = 0;
        for &name in zvec.iter() {
            result <<= 1;
            if self.wires.get(name).unwrap().state == WireState::True {
                result += 1;
            }
        }

        //dbg!(&self);
        println!("Result: {result}");
        result
    }
}

fn read_circuit(lines: &Vec<String>) -> Circuit {
    let mut circuit = Circuit::new();
    for line in lines {
        if !line.is_empty() {
            let wire = Wire::new(line);
            circuit.add_wire(&wire);
        }
    }
    //dbg!(&circuit);
    circuit
}

#[test]
fn test_prelim() {
    let result = read_circuit(&get_input("prelim.txt")).simulate();
    assert_eq!(result, 4);
}

#[test]
fn test_prelim2() {
    let result = read_circuit(&get_input("prelim2.txt")).simulate();
    assert_eq!(result, 2024);
}

#[test]
fn test_part1() {
    let result = read_circuit(&get_input("input.txt")).simulate();
    assert_eq!(result, 49574189473968);
}

fn main() {
    read_circuit(&get_input("prelim.txt")).simulate();
    read_circuit(&get_input("prelim2.txt")).simulate();
    read_circuit(&get_input("input.txt")).simulate();
}
