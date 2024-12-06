use std::collections::HashSet;
use std::fmt;
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

#[derive(Clone, PartialEq)]
enum GridPoint {
    Empty,
    Obstruction,
    NewObstruction,
    Visited,
    VisitedUpDown,
    VisitedLeftRight,
    VisitedUpDownLeftRight,
    VisitedStartUp,
}

impl fmt::Debug for GridPoint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmt, "."),
            Self::Obstruction => write!(fmt, "#"),
            Self::NewObstruction => write!(fmt, "O"),
            Self::Visited => write!(fmt, "X"),
            Self::VisitedUpDown => write!(fmt, "|"),
            Self::VisitedLeftRight => write!(fmt, "-"),
            Self::VisitedUpDownLeftRight => write!(fmt, "+"),
            Self::VisitedStartUp => write!(fmt, "^"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Coord {
    fn next_step(&self) -> Coord {
        let mut next = self.clone();
        match self.dir {
            Direction::Up => next.y -= 1,
            Direction::Down => next.y += 1,
            Direction::Left => next.x -= 1,
            Direction::Right => next.x += 1,
        }
        next
    }
}

#[derive(Clone)]
struct Grid {
    points: Vec<Vec<GridPoint>>,
    width: usize,
    height: usize,
    start: Coord,
    basic: bool,
}

impl Grid {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            width: 0,
            height: 0,
            start: Coord {
                x: 0,
                y: 0,
                dir: Direction::Up,
            },
            basic: false,
        }
    }

    fn create_grid(lines: &Vec<String>) -> Self {
        let mut grid = Grid::new();
        for line in lines {
            let mut row: Vec<GridPoint> = Vec::new();
            line.chars().for_each(|c| {
                let point = match c {
                    '.' => GridPoint::Empty,
                    '#' => GridPoint::Obstruction,
                    '^' => GridPoint::VisitedStartUp,
                    _ => panic!("Invalid character {c}"),
                };
                row.push(point);
            });
            grid.points.push(row);
        }

        grid.height = grid.points.len();
        if grid.height > 0 {
            grid.width = grid.points[0].len();
        }

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.points[y][x] == GridPoint::VisitedStartUp {
                    grid.start.x = x;
                    grid.start.y = y;
                    return grid;
                }
            }
        }
        panic!("Invalid grid with no start!");
    }

    fn is_on_edge(&self, cursor: &Coord) -> bool {
        match cursor.dir {
            Direction::Up => cursor.y == 0,
            Direction::Down => cursor.y == self.height - 1,
            Direction::Left => cursor.x == 0,
            Direction::Right => cursor.x == self.width - 1,
        }
    }

    fn collision(&self, cursor: &Coord) -> bool {
        match self.points[cursor.y][cursor.x] {
            GridPoint::Obstruction | GridPoint::NewObstruction => true,
            _ => false,
        }
    }

    fn mark(&mut self, cursor: &Coord) {
        if cursor.x >= self.width || cursor.y >= self.height {
            panic!("Invalid coord {:?} for grid {:?}", &cursor, &self);
        }
        let point = &mut self.points[cursor.y][cursor.x];
        if self.basic {
            *point = GridPoint::Visited;
        } else {
            *point = match cursor.dir {
                Direction::Up | Direction::Down => match point {
                    GridPoint::VisitedLeftRight | GridPoint::VisitedUpDownLeftRight => {
                        GridPoint::VisitedUpDownLeftRight
                    }
                    GridPoint::VisitedStartUp => GridPoint::VisitedStartUp,
                    _ => GridPoint::VisitedUpDown,
                },
                Direction::Left | Direction::Right => match point {
                    GridPoint::VisitedUpDown | GridPoint::VisitedUpDownLeftRight => {
                        GridPoint::VisitedUpDownLeftRight
                    }
                    GridPoint::VisitedStartUp => GridPoint::VisitedStartUp,
                    _ => GridPoint::VisitedLeftRight,
                },
            }
        }
    }

    fn guard_visit(&mut self) -> u32 {
        //dbg!(&self);
        let mut cursor = self.start;
        while !self.is_on_edge(&cursor) {
            let next = cursor.next_step();
            if self.collision(&next) {
                cursor.dir = cursor.dir.next();
            } else {
                cursor = next;
            }
            self.mark(&cursor);
        }
        //dbg!(&cursor);
        //dbg!(&self);
        let mut visited = 0;
        for row in &self.points {
            for col in row {
                match col {
                    // Everything that isn't empty or an obstruction is "Visited".
                    GridPoint::Empty | GridPoint::Obstruction => {}
                    _ => visited += 1,
                }
            }
        }
        println!("Visited: {}", visited);
        visited
    }

    fn check_for_loop(&mut self, new_obstruction: &Coord) -> bool {
        let mut visited = HashSet::new();
        let mut cursor = self.start;
        self.points[new_obstruction.y][new_obstruction.x] = GridPoint::NewObstruction;
        while !self.is_on_edge(&cursor) {
            if visited.contains(&cursor) {
                //dbg!(&self);
                return true;
            } else {
                visited.insert(cursor);
            }
            let next = cursor.next_step();
            if self.collision(&next) {
                cursor.dir = cursor.dir.next();
            } else {
                cursor = next;
            }
            self.mark(&cursor);
        }

        false
    }

    // Since we are only placing 1 new obstruction, the only valid locations for placement would
    // be coordinates that we visit on the usual guard path. Thus we already eliminate a large
    // number of locations that we would need to try. We further cache failed attempts to place
    // an obstruction, since there are occasional redundant overlaps that we could skip.
    fn compute_possible_obstructions(&self) -> usize {
        let mut obstructions = HashSet::new();
        let mut failed = HashSet::new();
        let mut cursor = self.start;
        while !self.is_on_edge(&cursor) {
            let next = cursor.next_step();
            if self.collision(&next) {
                cursor.dir = cursor.dir.next();
            } else {
                let mut new_grid = self.clone();
                // Need to ignore direction on the obstruction, since we only count unique locations
                if !obstructions.contains(&(next.x, next.y)) && !failed.contains(&(next.x, next.y))
                {
                    if new_grid.check_for_loop(&next) {
                        obstructions.insert((next.x, next.y));
                    } else {
                        failed.insert((next.x, next.y));
                    }
                }
                cursor = next;
            }
        }
        println!("Obstructions: {}", obstructions.len());
        obstructions.len()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "")?;
        writeln!(fmt, "width: {}", self.width)?;
        writeln!(fmt, "height: {}", self.height)?;
        writeln!(fmt, "start: {:?}", self.start)?;
        writeln!(fmt, "basic: {}", self.basic)?;
        for row in &self.points {
            for col in row {
                write!(fmt, "{:?}", col)?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}

#[test]
fn test_prelim() {
    let unique = Grid::create_grid(&get_input("prelim.txt")).guard_visit();
    assert_eq!(unique, 41);
}

#[test]
fn test_prelim2() {
    let obstructions = Grid::create_grid(&get_input("prelim.txt")).compute_possible_obstructions();
    assert_eq!(obstructions, 6);
}

#[test]
fn test_part1() {
    let unique = Grid::create_grid(&get_input("input.txt")).guard_visit();
    assert_eq!(unique, 5404);
}

#[test]
fn test_part2() {
    let obstructions = Grid::create_grid(&get_input("input.txt")).compute_possible_obstructions();
    assert_eq!(obstructions, 1984);
}

fn main() {
    Grid::create_grid(&get_input("prelim.txt")).guard_visit();
    Grid::create_grid(&get_input("input.txt")).guard_visit();
    Grid::create_grid(&get_input("prelim.txt")).compute_possible_obstructions();
    Grid::create_grid(&get_input("input.txt")).compute_possible_obstructions();
}
