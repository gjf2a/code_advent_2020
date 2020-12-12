use advent_code_lib::{Position, Dir, for_each_line, normalize_degrees};
use std::io;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let mut s = Ship::new();
    for_each_line(filename, |line| Ok({
        interpret_move_puzzle_1(&mut s, line);
    }))?;
    Ok(manhattan_str(s.p))
}

pub fn manhattan_str(p: Position) -> String {
    (p.col.abs() + p.row.abs()).to_string()
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let mut s = Ship::new();
    let mut waypoint = Position::from((10, -1));
    for_each_line(filename, |line| Ok({
        interpret_move_puzzle_2(&mut s, &mut waypoint, line);
    }))?;
    Ok(manhattan_str(s.p))
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Ship {
    p: Position,
    heading: Dir
}

impl Ship {
    pub fn new() -> Self {
        Ship {p: Position {col: 0, row: 0}, heading: Dir::E}
    }

    pub fn go(&mut self, dir: Dir, dist: isize) {
        position_jump(&mut self.p, dir, dist);
    }

    pub fn turn(&mut self, degrees: isize) {
        self.heading = self.heading.rotated_degrees(degrees);
    }
}

pub fn position_jump(p: &mut Position, dir: Dir, dist: isize) {
    *p += dir.position_offset() * dist;
}

pub fn rotate_waypoint(p: &mut Position, degrees: isize) {
    let steps_right = normalize_degrees(degrees) / 90;
    for _ in 0..steps_right {
        *p = Position { col: -p.row, row: p.col };
    }
}

pub fn decode_line(line: &str) -> (char,isize) {
    let mut chars = line.chars();
    let instruction = chars.next().unwrap();
    let value = chars.collect::<String>().parse::<isize>().unwrap();
    (instruction, value)
}

pub fn interpret_move_puzzle_1(s: &mut Ship, line: &str) {
    let (instruction, value) = decode_line(line);
    match instruction {
        'N' => s.go(Dir::N, value),
        'S' => s.go(Dir::S, value),
        'E' => s.go(Dir::E, value),
        'W' => s.go(Dir::W, value),
        'F' => s.go(s.heading, value),
        'L' => s.turn(-value),
        'R' => s.turn(value),
        _ => panic!("Unrecognized instruction")
    }
}

pub fn interpret_move_puzzle_2(s: &mut Ship, w: &mut Position, line: &str) {
    let (instruction, value) = decode_line(line);
    match instruction {
        'N' => position_jump(w, Dir::N, value),
        'S' => position_jump(w, Dir::S, value),
        'E' => position_jump(w, Dir::E, value),
        'W' => position_jump(w, Dir::W, value),
        'F' => s.p += *w * value,
        'L' => rotate_waypoint(w, -value),
        'R' => rotate_waypoint(w, value),
        _ => panic!("Unrecognized instruction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_1() {
        assert_eq!(solve_1("day_12_example.txt").unwrap(), "25");
    }

    #[test]
    pub fn test_solution_1() {
        assert_eq!(solve_1("day_12_input.txt").unwrap(), "441");
    }

    #[test]
    pub fn test_2() {
        assert_eq!(solve_2("day_12_example.txt").unwrap(), "286");
    }

    #[test]
    pub fn test_rotate_waypoint() {
        let mut waypoint = Position::from((10, -4));
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!(waypoint, Position::from((4, 10)));
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!(waypoint, Position::from((-10, 4)));
    }
}