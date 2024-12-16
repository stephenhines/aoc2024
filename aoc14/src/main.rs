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

const PRE_WIDTH: usize = 11;
const PRE_HEIGHT: usize = 7;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Velocity {
    dx: isize,
    dy: isize,
}

#[derive(Debug)]
struct Robot {
    pos: Coord,
    vel: Velocity,
}

impl Robot {
    fn new(line: &String) -> Self {
        let toks: Vec<&str> = line.split_ascii_whitespace().collect();
        let p_part: Vec<&str> = toks[0].split("=").collect();
        let v_part: Vec<&str> = toks[1].split("=").collect();
        let pos: Vec<&str> = p_part[1].split(",").collect();
        let vel: Vec<&str> = v_part[1].split(",").collect();
        let x = pos[0].parse::<usize>().unwrap();
        let y = pos[1].parse::<usize>().unwrap();
        let dx = vel[0].parse::<isize>().unwrap();
        let dy = vel[1].parse::<isize>().unwrap();
        Self {
            pos: Coord { x, y },
            vel: Velocity { dx, dy },
        }
    }

    fn advance(&mut self, time: usize, width: usize, height: usize) {
        if self.vel.dx > 0 {
            let dx = (self.vel.dx as usize * time) % width;
            self.pos.x = (self.pos.x + dx) % width;
        } else {
            let neg_dx = (self.vel.dx * -1) as usize;
            let neg_dx = (neg_dx * time) % width;
            if neg_dx > self.pos.x {
                let dx = width - neg_dx;
                self.pos.x += dx;
            } else {
                self.pos.x -= neg_dx;
            }
        }

        if self.vel.dy > 0 {
            let dy = (self.vel.dy as usize * time) % height;
            self.pos.y = (self.pos.y + dy) % height;
        } else {
            let neg_dy = (self.vel.dy * -1) as usize;
            let neg_dy = (neg_dy * time) % height;
            if neg_dy > self.pos.y {
                let dy = height - neg_dy;
                self.pos.y += dy;
            } else {
                self.pos.y -= neg_dy;
            }
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Grid {
    fn create_grid(width: usize, height: usize, lines: &Vec<String>) -> Self {
        let mut robots = Vec::new();
        for line in lines {
            let robot = Robot::new(line);
            robots.push(robot);
        }
        Grid {
            width,
            height,
            robots,
        }
    }

    fn advance_time(&mut self, time: usize) -> &mut Self {
        self.robots
            .iter_mut()
            .for_each(|r| r.advance(time, self.width, self.height));
        self
    }

    // Get the quadrant number for a given robot. Returns 1, 2, 3, or 4 for
    // the normal quadrants, and 0 if we're on a midpoint line.
    fn get_quadrant(&self, robot: &Robot) -> usize {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;
        let (x, y) = (robot.pos.x, robot.pos.y);
        if x == mid_x || y == mid_y {
            return 0;
        }
        if x > mid_x {
            if y < mid_y {
                return 1;
            } else {
                return 4;
            }
        } else {
            if y < mid_y {
                return 2;
            } else {
                return 3;
            }
        }
    }

    fn compute_safety(&self) -> usize {
        let mut quadrants: [usize; 5] = [0; 5];
        self.robots.iter().for_each(|r| {
            quadrants[self.get_quadrant(r)] += 1;
        });

        dbg!(quadrants);

        let safety = quadrants[1] * quadrants[2] * quadrants[3] * quadrants[4];
        println!("Safety: {}", safety);
        safety
    }

    fn find_tree(&mut self) -> usize {
        let mut iter = 83; // Discovered after looking at first several hundred inputs and seeing large vertical bands
        self.advance_time(83);
        while iter < self.width * self.height {
            dbg!(iter);
            dbg!(&self);

            self.advance_time(self.width);
            iter += self.width;
        }
        0
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "")?;
        let mut points: [[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];
        for robot in &self.robots {
            points[robot.pos.y][robot.pos.x] = '*';
        }
        for y in 0..self.height {
            for x in 0..self.width {
                write!(fmt, "{}", points[y][x])?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}

#[test]
fn test_prelim() {
    let safety = Grid::create_grid(PRE_WIDTH, PRE_HEIGHT, &get_input("prelim.txt"))
        .advance_time(100)
        .compute_safety();
    assert_eq!(safety, 12);
}

#[test]
fn test_part1() {
    let safety = Grid::create_grid(WIDTH, HEIGHT, &get_input("input.txt"))
        .advance_time(100)
        .compute_safety();
    assert_eq!(safety, 218619324);
}

fn main() {
    Grid::create_grid(PRE_WIDTH, PRE_HEIGHT, &get_input("prelim.txt"))
        .advance_time(100)
        .compute_safety();
    Grid::create_grid(WIDTH, HEIGHT, &get_input("input.txt"))
        .advance_time(100)
        .compute_safety();
    let i = Grid::create_grid(WIDTH, HEIGHT, &get_input("input.txt")).find_tree();
    println!("i: {}", i);
    let mut grid = Grid::create_grid(WIDTH, HEIGHT, &get_input("input.txt"));
    grid.advance_time(6446);
    println!("{:?}", grid);
}
