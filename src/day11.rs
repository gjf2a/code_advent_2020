use std::{io, mem};
use advent_code_lib::{all_lines, DirIter, Dir, Position};
use std::fmt::{Display, Formatter, Error};
use Rule::*;

const OCCUPIED: char = '#';
const FLOOR: char = '.';
const EMPTY: char = 'L';

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(GameOfSeats::num_occupied_at_stable(filename, Puzzle1)?.to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(GameOfSeats::num_occupied_at_stable(filename, Puzzle2)?.to_string())
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum Rule {
    Puzzle1, Puzzle2
}

impl Rule {
    pub fn too_many_people(&self, num_people: usize) -> bool {
        num_people >= match self { Puzzle1 => 4, Puzzle2 => 5}
    }

    pub fn seat_occupied_in(&self, gos: &GameOfSeats, d: Dir, p: Position) -> bool {
        self.projected_seat(gos, d, p) == OCCUPIED
    }

    pub fn projected_seat(&self, gos: &GameOfSeats, d: Dir, p: Position) -> char {
        gos.seat(match self {
            Puzzle1 => p.updated(d),
            Puzzle2 => Rule::puzzle2projection(gos, d, p)
        })
    }

    pub fn puzzle2projection(gos: &GameOfSeats, d: Dir, p: Position) -> Position {
        let mut p = p;
        loop {
            p.update(d);
            if !gos.within_outer_ring(p) || gos.seat(p) != FLOOR {
                return p;
            }
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct GameOfSeats {
    seating: Vec<Vec<char>>,
    rule: Rule
}

impl Display for GameOfSeats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.seating.iter()
            .map(|line| format!("{}\n", line.iter().collect::<String>()))
            .collect::<String>())
    }
}

impl GameOfSeats {
    pub fn from(filename: &str, rule: Rule) -> io::Result<Self> {
        Ok(GameOfSeats {
            seating: all_lines(filename)?
                .map(|line| line.unwrap().chars().collect())
                .collect(),
            rule})
    }

    pub fn num_occupied_at_stable(filename: &str, rule: Rule) -> io::Result<usize> {
        Ok(GameOfSeats::from(filename, rule)?.stable_state().num_occupied())
    }

    pub fn height(&self) -> usize {self.seating.len()}
    pub fn width(&self) -> usize {self.seating[0].len()}

    pub fn seat(&self, p: Position) -> char {
        if self.in_bounds(p) {
            self.seating[p.row as usize][p.col as usize]
        } else {
            FLOOR
        }
    }

    pub fn within_outer_ring(&self, p: Position) -> bool {
        p.col >= -1 && p.row >= -1 && p.col <= self.width() as isize && p.row <= self.height() as isize
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        p.col >= 0 && p.row >= 0 && self.in_bounds_u(p.col as usize, p.row as usize)
    }

    pub fn in_bounds_u(&self, col: usize, row: usize) -> bool {
        col < self.width() && row < self.height()
    }

    pub fn num_adj_occupied(&self, p: Position) -> Option<usize> {
        if self.in_bounds(p) {
            Some(DirIter::new()
                .filter(|d| self.rule.seat_occupied_in(self, *d, p))
                .count())
        } else {
            None
        }
    }

    pub fn iter(&self) -> GameOfSeatsIterator {
        GameOfSeatsIterator {gos: Some(self.clone())}
    }

    pub fn create_next(&self) -> GameOfSeats {
        GameOfSeats {
            seating: (0..self.height())
                .map(|row| (0..self.width())
                    .map(|col| self.iterated_seat_at(Position {col: col as isize, row: row as isize}))
                    .collect())
                .collect(),
            rule: self.rule
        }
    }

    pub fn iterated_seat_at(&self, p: Position) -> char {
        let seat = self.seat(p);
        let adj = self.num_adj_occupied(p).unwrap();
        if seat == EMPTY && adj == 0 {OCCUPIED}
        else if seat == OCCUPIED && self.rule.too_many_people(adj) {EMPTY}
        else { seat }
    }

    pub fn stable_state(&self) -> GameOfSeats {
        self.iter().last().unwrap()
    }

    pub fn num_occupied(&self) -> usize {
        self.seating.iter()
            .map(|row| row.iter()
                .filter(|s| **s == OCCUPIED)
                .count())
            .sum()
    }
}

pub struct GameOfSeatsIterator {
    gos: Option<GameOfSeats>
}

impl Iterator for GameOfSeatsIterator {
    type Item = GameOfSeats;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gos != None {
            let gos = self.gos.as_ref().unwrap();
            let next = gos.create_next();
            if next == *gos {
                self.gos = None;
                return Some(next);
            } else {
                let mut result = Some(next);
                mem::swap(&mut result, &mut self.gos);
                return result;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let start = GameOfSeats::from("in/day11_ex1.txt", Puzzle1).unwrap();
        assert_eq!(start.to_string(), EXPECTED_1[0]);
    }

    #[test]
    fn test_example_1() -> io::Result<()> {
        test_example(Puzzle1, &EXPECTED_1)
    }

    #[test]
    fn test_example_2() -> io::Result<()> {
        test_example(Puzzle2, &EXPECTED_2)
    }

    fn test_example(rule: Rule, targets: &[&str]) -> io::Result<()> {
        let start = GameOfSeats::from("in/day11_ex1.txt", rule)?;
        let mut iter = start.iter();
        for i in 0..targets.len() {
            println!("Testing target {}", i);
            assert_eq!(iter.next().unwrap().to_string(), targets[i]);
        }
        assert!(iter.next() == None);
        Ok(())
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("in/day11_ex1.txt").unwrap(), "37");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("in/day11_ex1.txt").unwrap(), "26");
    }

    #[test]
    fn test_solve_2_corners() {
        let mut gos = GameOfSeats::from("in/day11_ex1.txt", Puzzle2).unwrap();
        let c = Position {row: 0, col: 0};
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), FLOOR);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 0);
        assert_eq!(gos.iterated_seat_at(c), OCCUPIED);
        gos = gos.create_next();
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), FLOOR);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 3);
        assert_eq!(gos.iterated_seat_at(c), OCCUPIED);
        gos = gos.create_next();
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), FLOOR);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 1);
        assert_eq!(gos.seat(c), OCCUPIED);
    }

    const EXPECTED_1: [&'static str; 6] = [
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
        "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
",
        "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
",
        "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
",
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
"];

    const EXPECTED_2: [&'static str; 7] = [
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
"#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
",
"#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
",
"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
",
"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
",
"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"];

}