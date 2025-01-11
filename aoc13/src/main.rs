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

type Coord = (usize, usize);

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

#[derive(Debug)]
struct Claw {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl Claw {
    fn new(button_a: Coord, button_b: Coord, prize: Coord) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    // Find the lowest cost in A/B button presses
    fn get_cost(&self) -> usize {
        let mut lowest_cost = usize::MAX;

        // B is cheaper to press than A, so we'll start out with lower A
        // We cap at 100 presses.
        for a in 0..=100 {
            let ax = a * self.button_a.0;
            let ay = a * self.button_a.1;
            if ax > self.prize.0 || ay > self.prize.1 {
                // We're done when A presses have exceeded either bound
                break;
            }
            // We don't have to brute force anything at all, because we can just do % and / to compute things
            let target_x = self.prize.0 - ax;
            let target_y = self.prize.1 - ay;
            if target_x % self.button_b.0 == 0 {
                let b = target_x / self.button_b.0;
                let by = b * self.button_b.1;
                if by == target_y {
                    let cost = a * BUTTON_A_COST + b * BUTTON_B_COST;
                    if cost < lowest_cost {
                        lowest_cost = cost;
                        if b > 100 {
                            panic!("Unexpected B presses {b}");
                        }
                    }
                }
            }
        }

        if lowest_cost == usize::MAX {
            lowest_cost = 0;
        }

        //println!("claw cost {lowest_a} {lowest_b}: {lowest_cost}");
        lowest_cost
    }
}

fn get_total_cost(claws: &[Claw]) -> usize {
    let mut cost = 0;

    for claw in claws {
        cost += claw.get_cost();
    }

    println!("Total cost: {cost}");
    cost
}

fn read_claws(lines: &[String]) -> Vec<Claw> {
    let mut claws = Vec::new();

    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    for line in lines {
        let toks = line.split(": ").collect::<Vec<_>>();
        match toks[0] {
            "Button A" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('+').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let y = ctoks[1].split('+').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                button_a = (x, y);
            }
            "Button B" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('+').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let y = ctoks[1].split('+').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                button_b = (x, y);
            }
            "Prize" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('=').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let y = ctoks[1].split('=').collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let prize = (x, y);
                let claw = Claw::new(button_a, button_b, prize);
                claws.push(claw);
            }
            "" => {}
            _ => {
                panic!("Invalid input: {line}");
            }
        }
    }

    claws
}

#[test]
fn test_prelim() {
    let cost = get_total_cost(&read_claws(&get_input("prelim.txt")));
    assert_eq!(cost, 480);
}

#[test]
fn test_part1() {
    let cost = get_total_cost(&read_claws(&get_input("input.txt")));
    assert_eq!(cost, 39748);
}

fn main() {
    get_total_cost(&read_claws(&get_input("prelim.txt")));
    get_total_cost(&read_claws(&get_input("input.txt")));
}
