use advent_code_lib::{Position, Dir, for_each_line};
use std::io;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let mut s = Ship::new();
    for_each_line(filename, |line| Ok({
        interpret_move_puzzle_1(&mut s, line);
    }))?;
    Ok((s.p.col.abs() + s.p.row.abs()).to_string())
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

pub fn interpret_move_puzzle_1(s: &mut Ship, line: &str) {
    let mut chars = line.chars();
    let instruction = chars.next().unwrap();
    let value = chars.collect::<String>().parse::<isize>().unwrap();
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

}

#[cfg(test)]
mod tests {
    use crate::day12::solve_1;

    #[test]
    pub fn test_1() {
        assert_eq!(solve_1("day_12_example.txt").unwrap(), "25");
    }

    #[test]
    pub fn test_solution_1() {
        assert_eq!(solve_1("day_12_input.txt").unwrap(), "441");
    }
}