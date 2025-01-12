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

type Coord = (isize, isize);

const BUTTON_A_COST: isize = 3;
const BUTTON_B_COST: isize = 1;

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

    /*
        Find the lowest cost in A/B button presses
        Solve the pair of equations instead of brute forcing.

        a * ax + b * bx = px                            (1)
        a * ay + b * by = py                            (2)

        Hooray for first principles!

            # start with (1)
        a * ax + b * bx = px                            (1)
            # subtract b * bx
        a * ax = px - b * bx
            # divide by ax
        a = (px - b * bx) / ax                          (3)

            # substitute (3) into (2)
        (px - b * bx) / ax * ay + b * by = py
            # multiply by ax
        (px - b * bx) * ay + b * by * ax = py * ax
            # expand
        px * ay - b * bx * ay + b * by * ax = py * ax
            # subtract px * ay
        - b * bx * ay + b * by * ax = py * ax - px * ay
            # factor out b on the lhs
        b * (by * ax - bx * ay) = py * ax - px * ay
            # divide by lhs (other than b term)
        b = (py * ax - px * ay) / (by * ax - bx * ay)   (4)

        # This gives us the following two final equations, which we can use.
        # We need to later check that we got positive, integral solutions,
        # which is easy to verify by plugging everything back into the first
        # equations and checking versus the prize coordinates.

        b = (py * ax - px * ay) / (by * ax - bx * ay)   (4)
        a = (px - b * bx) / ax                          (3)
    */
    fn get_cost(&self) -> isize {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let b = (py * ax - px * ay) / (by * ax - bx * ay);
        let a = (px - b * bx) / ax;

        // Probably redundant since there are no negative values
        if a < 0 || b < 0 {
            return 0;
        }

        // Check to ensure it works
        let total = (a * ax + b * bx, a * ay + b * by);
        //println!("prize: {:?} - a: {a} b: {b}", self.prize);
        if total == self.prize {
            a * BUTTON_A_COST + b * BUTTON_B_COST
        } else {
            0
        }
    }
}

fn get_total_cost(claws: &[Claw]) -> isize {
    let cost = claws.iter().map(|c| c.get_cost()).sum();
    println!("Total cost: {cost}");
    cost
}

fn read_claws(lines: &[String], fixed: bool) -> Vec<Claw> {
    let mut claws = Vec::new();

    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    for line in lines {
        let toks = line.split(": ").collect::<Vec<_>>();
        match toks[0] {
            "Button A" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('+').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                let y = ctoks[1].split('+').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                button_a = (x, y);
            }
            "Button B" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('+').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                let y = ctoks[1].split('+').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                button_b = (x, y);
            }
            "Prize" => {
                let ctoks = toks[1].split(", ").collect::<Vec<_>>();
                let x = ctoks[0].split('=').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                let y = ctoks[1].split('=').collect::<Vec<_>>()[1]
                    .parse::<isize>()
                    .unwrap();
                let mut prize = (x, y);
                if fixed {
                    prize.0 += 10000000000000;
                    prize.1 += 10000000000000;
                }
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
    let cost = get_total_cost(&read_claws(&get_input("prelim.txt"), false));
    assert_eq!(cost, 480);
}

#[test]
fn test_part1() {
    let cost = get_total_cost(&read_claws(&get_input("input.txt"), false));
    assert_eq!(cost, 39748);
}

#[test]
fn test_part2() {
    let cost = get_total_cost(&read_claws(&get_input("input.txt"), true));
    assert_eq!(cost, 74478585072604);
}

fn main() {
    get_total_cost(&read_claws(&get_input("prelim.txt"), false));
    get_total_cost(&read_claws(&get_input("input.txt"), false));
    get_total_cost(&read_claws(&get_input("prelim.txt"), true));
    get_total_cost(&read_claws(&get_input("input.txt"), true));
}
