use advent_code_lib::{Position, Dir, for_each_line, normalize_degrees, DirType};
use std::io;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let mut ship_pos = Position::new();
    let mut ship_heading = Dir::E;
    for_each_line(filename, |line| Ok({
        interpret_move_puzzle_1(&mut ship_pos, &mut ship_heading, line);
    }))?;
    Ok(manhattan_str(ship_pos))
}

pub fn manhattan_str(p: Position) -> String {
    (p.col.abs() + p.row.abs()).to_string()
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let mut ship = Position::new();
    let mut waypoint = Position::from((10, -1));
    for_each_line(filename, |line| Ok({
        interpret_move_puzzle_2(&mut ship, &mut waypoint, line);
    }))?;
    Ok(manhattan_str(ship))
}

pub fn jump(p: &mut Position, dir: Dir, dist: isize) {
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

pub fn interpret_move_puzzle_1(ship_pos: &mut Position, ship_heading: &mut Dir, line: &str) {
    let (instruction, value) = decode_line(line);
    match instruction {
        'N' => jump(ship_pos, Dir::N, value),
        'S' => jump(ship_pos, Dir::S, value),
        'E' => jump(ship_pos, Dir::E, value),
        'W' => jump(ship_pos, Dir::W, value),
        'F' => jump(ship_pos, *ship_heading, value),
        'L' => *ship_heading = ship_heading.rotated_degrees(-value),
        'R' => *ship_heading = ship_heading.rotated_degrees(value),
        _ => panic!("Unrecognized instruction")
    }
}

pub fn interpret_move_puzzle_2(s: &mut Position, w: &mut Position, line: &str) {
    let (instruction, value) = decode_line(line);
    match instruction {
        'N' => jump(w, Dir::N, value),
        'S' => jump(w, Dir::S, value),
        'E' => jump(w, Dir::E, value),
        'W' => jump(w, Dir::W, value),
        'F' => *s += *w * value,
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
        assert_eq!(solve_1("in/day12_ex.txt").unwrap(), "25");
    }

    #[test]
    pub fn test_solution_1() {
        assert_eq!(solve_1("in/day12.txt").unwrap(), "441");
    }

    #[test]
    pub fn test_2() {
        assert_eq!(solve_2("in/day12_ex.txt").unwrap(), "286");
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