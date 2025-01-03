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

/*
   +---+---+---+
   | 7 | 8 | 9 |
   +---+---+---+           +---+---+
   | 4 | 5 | 6 |           | ^ | A |
   +---+---+---+       +---+---+---+
   | 1 | 2 | 3 |       | < | v | > |
   +---+---+---+       +---+---+---+
       | 0 | A |
       +---+---+
*/
#[rustfmt::skip]
const MOVES_0A: [[&str; 11]; 11] = [
    //                                        ******** TO ********
    //       0,       1,      2,       3,       4,      5,       6,        7,       8,        9,        A
    [      "A",   "^<A",   "^A",   "^>A",  "^^<A",  "^^A",  "^^>A",  "^^^<A",  "^^^A",  "^^^>A",     ">A" ],  // 0
    [    ">vA",     "A",   ">A",   ">>A",    "^A",  "^>A",  "^>>A",    "^^A",  "^^>A",  "^^>>A",   ">>vA" ],  // 1
    [     "vA",    "<A",    "A",    ">A",   "<^A",   "^A",   "^>A",   "<^^A",   "^^A",   "^^>A",    "v>A" ],  // 2    **
    [    "<vA",   "<<A",   "<A",     "A",  "<<^A",  "<^A",    "^A",  "<<^^A",  "<^^A",    "^^A",     "vA" ],  // 3    **
    [   ">vvA",    "vA",  "v>A",  "v>>A",     "A",   ">A",   ">>A",     "^A",   "^>A",   "^>>A",  ">>vvA" ],  // 4    **
    [    "vvA",   "<vA",   "vA",   "v>A",    "<A",    "A",    ">A",    "<^A",    "^A",    "^>A",   "vv>A" ],  // 5   FROM
    [   "<vvA",  "<<vA",  "<vA",    "vA",   "<<A",   "<A",     "A",   "<<^A",   "<^A",     "^A",    "vvA" ],  // 6    **
    [  ">vvvA",   "vvA", "vv>A", "vv>>A",    "vA",  "v>A",  "v>>A",      "A",    ">A",    ">>A", ">>vvvA" ],  // 7    **
    [   "vvvA",  "<vvA",  "vvA",  "vv>A",   "<vA",   "vA",   "v>A",     "<A",     "A",     ">A",  "vvv>A" ],  // 8    **
    [  "<vvvA", "<<vvA", "<vvA",   "vvA",  "<<vA",  "<vA",    "vA",    "<<A",    "<A",      "A",   "vvvA" ],  // 9
    [     "<A",  "^<<A",  "<^A",    "^A", "^^<<A", "<^^A",   "^^A", "^^^<<A", "<^^^A",   "^^^A",      "A" ],  // A
];

/*
 ^ 0       +---+---+
 < 1       | ^ | A |
 v 2   +---+---+---+
 > 3   | < | v | > |
 A 4   +---+---+---+
*/
#[rustfmt::skip]
const MOVES_LUDRA: [[&str; 5]; 5] = [
    //       ******** TO ********
    //    ^,      <,     v,     >,      A
    [   "A",  "v<A",  "vA", "v>A",   ">A", ],  // ^
    [ ">^A",    "A",  ">A", ">>A", ">>^A", ],  // <   ****
    [  "^A",   "<A",   "A",  ">A",  "^>A", ],  // v   FROM
    [ "<^A",  "<<A",  "<A",   "A",   "^A", ],  // >   ****
    [  "<A", "v<<A", "<vA",  "vA",    "A", ],  // A
];

#[rustfmt::skip]
#[allow(dead_code)]
const MOVES_0A_NUM: [[usize; 11]; 11] = [
    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A
    [  0, 2, 1, 2, 3, 2, 3, 4, 3, 4, 1 ], // 0
    [  2, 0, 1, 2, 1, 2, 3, 2, 3, 4, 3 ], // 1
    [  1, 1, 0, 1, 2, 1, 2, 3, 2, 3, 2 ], // 2
    [  2, 2, 1, 0, 3, 2, 1, 4, 3, 2, 1 ], // 3
    [  3, 1, 2, 3, 0, 1, 2, 1, 2, 3, 4 ], // 4
    [  2, 2, 1, 2, 1, 0, 1, 2, 1, 2, 3 ], // 5
    [  3, 3, 2, 1, 2, 1, 0, 3, 2, 1, 2 ], // 6
    [  4, 2, 3, 4, 1, 2, 3, 0, 1, 2, 5 ], // 7
    [  3, 3, 2, 3, 2, 1, 2, 1, 0, 1, 4 ], // 8
    [  4, 4, 3, 2, 3, 2, 1, 2, 1, 0, 3 ], // 9
    [  1, 3, 2, 1, 4, 3, 2, 5, 4, 3, 0 ], // A
];

/*
       +---+---+
       | ^ | A |
   +---+---+---+
   | < | v | > |
   +---+---+---+
*/

struct NumPadRobot {
    loc: usize,
}

impl NumPadRobot {
    fn new() -> Self {
        // Starts at the 'A' button
        NumPadRobot { loc: 10 }
    }

    fn go_to(&mut self, to: usize) -> &str {
        let moves = MOVES_0A[self.loc][to];
        self.loc = to;
        moves
    }

    fn go_to_char(&mut self, to_char: char) -> &str {
        let to = match to_char {
            '0'..='9' => to_char.to_digit(10).unwrap() as usize,
            'A' => 10,
            _ => {
                panic!("Invalid entry: {to_char}");
            }
        };

        self.go_to(to)
    }
}

struct DirPadRobot {
    loc: usize,
}

impl DirPadRobot {
    fn new() -> Self {
        DirPadRobot { loc: 4 }
    }

    fn go_to(&mut self, to: usize) -> &str {
        let moves = MOVES_LUDRA[self.loc][to];
        self.loc = to;
        moves
    }

    fn go_to_char(&mut self, to_char: char) -> &str {
        let to = match to_char {
            '^' => 0,
            '<' => 1,
            'v' => 2,
            '>' => 3,
            'A' => 4,
            _ => {
                panic!("Invalid entry: {to_char}");
            }
        };

        self.go_to(to)
    }
}

fn calculate_moves(line: &str) -> String {
    let mut numpad = NumPadRobot::new();
    let mut output = String::new();
    for c in line.chars() {
        output.push_str(numpad.go_to_char(c));
    }
    println!("Code: {line} part 1 moves: {output}");

    let mut dirpad = DirPadRobot::new();
    let mut out2 = String::new();
    for c in output.chars() {
        out2.push_str(dirpad.go_to_char(c));
    }
    println!("Code: {line} part 2 moves: {out2}");

    let mut dirpad2 = DirPadRobot::new();
    let mut out3 = String::new();
    for c in out2.chars() {
        out3.push_str(dirpad2.go_to_char(c));
    }
    println!("Code: {line} part 3 moves {}: {out3}", out3.len());

    out3
}

#[allow(dead_code)]
fn calculate_moves_all(lines: &Vec<String>) -> Vec<String> {
    let mut output_codes = Vec::new();
    for line in lines {
        output_codes.push(calculate_moves(line));
    }
    output_codes
}

fn calculate_complexity(lines: &Vec<String>) -> usize {
    let mut total_complexity = 0;
    for line in lines {
        let toks = line.split('A').collect::<Vec<_>>();
        let numeric = toks[0].to_string().parse::<usize>().unwrap();
        let moves = calculate_moves(line);
        let num_moves = moves.len();
        let complexity = numeric * num_moves;
        total_complexity += complexity;
    }

    println!("Total complexity: {total_complexity}");
    total_complexity
}

// Verify that MOVES is perfectly symmetric (even if I got it wrong lol)
#[test]
fn test_moves() {
    for f in 0..11 {
        for t in 0..11 {
            assert_eq!(MOVES_0A_NUM[f][t], MOVES_0A_NUM[t][f]);
            assert_eq!(MOVES_0A[f][t].len(), MOVES_0A[t][f].len());
            assert_eq!(MOVES_0A[f][t].len(), MOVES_0A_NUM[f][t] + 1);
        }
    }
}

#[test]
fn test_prelim_part1() {
    let output = calculate_moves("029A");
    assert_eq!(
        output.len(),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    );

    let output = calculate_moves("980A");
    assert_eq!(
        output.len(),
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
    );
}

#[test]
fn test_prelim() {
    let complexity = calculate_complexity(&get_input("prelim.txt"));
    assert_eq!(complexity, 126384);
}

#[test]
fn test_part1() {
    let complexity = calculate_complexity(&get_input("input.txt"));
    assert_eq!(complexity, 248684);
}

fn main() {
    calculate_complexity(&get_input("prelim.txt"));
    calculate_complexity(&get_input("input.txt"));
}
