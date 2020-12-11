use std::{io, mem};
use advent_code_lib::{all_lines, DirIter, Dir, Position};
use std::fmt::{Display, Formatter, Error};
use Rule::*;

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
        self.projected_seat(gos, d, p) == Seat::Occupied
    }

    pub fn projected_seat(&self, gos: &GameOfSeats, d: Dir, p: Position) -> Seat {
        gos.seat(match self {
            Puzzle1 => p.updated(d),
            Puzzle2 => Rule::puzzle2projection(gos, d, p)
        })
    }

    pub fn puzzle2projection(gos: &GameOfSeats, d: Dir, p: Position) -> Position {
        let mut p = p;
        loop {
            p.update(d);
            if !gos.within_outer_ring(p) || gos.seat(p) != Seat::Floor {
                return p;
            }
        }
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum Seat {
    Empty, Occupied, Floor
}

impl Seat {
    pub fn from(c: char) -> Option<Self> {
        match c {
            'L' => Some(Seat::Empty),
            '.' => Some(Seat::Floor),
            '#' => Some(Seat::Occupied),
            _ => None
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Seat::Empty => 'L',
            Seat::Floor => '.',
            Seat::Occupied => '#'
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct GameOfSeats {
    seating: Vec<Vec<Seat>>,
    rule: Rule
}

impl Display for GameOfSeats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.seating.iter()
            .map(|line| format!("{}\n", line.iter()
                .map(|s| s.as_char()).collect::<String>()))
            .collect::<String>())
    }
}

impl GameOfSeats {
    pub fn from(filename: &str, rule: Rule) -> io::Result<Self> {
        Ok(GameOfSeats {
            seating: all_lines(filename)?
                .map(|line| line.unwrap().chars()
                    .map(|c| Seat::from(c).unwrap())
                    .collect())
                .collect(),
            rule})
    }

    pub fn num_occupied_at_stable(filename: &str, rule: Rule) -> io::Result<usize> {
        Ok(GameOfSeats::from(filename, rule)?.stable_state().num_occupied())
    }

    pub fn height(&self) -> usize {self.seating.len()}
    pub fn width(&self) -> usize {self.seating[0].len()}

    pub fn seat(&self, p: Position) -> Seat {
        if self.in_bounds_i(p.col, p.row) {
            self.seating[p.row as usize][p.col as usize]
        } else {
            Seat::Floor
        }
    }

    pub fn within_outer_ring(&self, p: Position) -> bool {
        p.col >= -1 && p.row >= -1 && p.col <= self.width() as isize && p.row <= self.height() as isize
    }

    pub fn in_bounds(&self, p: Position) -> bool {
        self.in_bounds_i(p.col, p.row)
    }

    pub fn in_bounds_i(&self, col: isize, row: isize) -> bool {
        col >= 0 && row >= 0 && self.in_bounds_u(col as usize, row as usize)
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

    pub fn iterate(&self) -> GameOfSeats {
        GameOfSeats {
            seating: (0..self.height())
                .map(|row| (0..self.width())
                    .map(|col| self.iterated_seat_at(Position {col: col as isize, row: row as isize}))
                    .collect())
                .collect(),
            rule: self.rule
        }
    }

    pub fn iterated_seat_at(&self, p: Position) -> Seat {
        let seat = self.seat(p);
        let adj = self.num_adj_occupied(p).unwrap();
        if seat == Seat::Empty && adj == 0 {
            Seat::Occupied
        } else if seat == Seat::Occupied && self.rule.too_many_people(adj) {
            Seat::Empty
        } else { seat }
    }

    pub fn stable_state(&self) -> GameOfSeats {
        self.iter().last().unwrap()
    }

    pub fn num_occupied(&self) -> usize {
        self.seating.iter()
            .map(|row| row.iter()
                .filter(|s| **s == Seat::Occupied)
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
            let candidate = gos.iterate();
            if candidate == *gos {
                self.gos = None;
                return Some(candidate);
            } else {
                let mut candidate = Some(candidate);
                mem::swap(&mut candidate, &mut self.gos);
                return candidate;
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
        let start = GameOfSeats::from("day_11_example_1.txt", Puzzle1).unwrap();
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
        let start = GameOfSeats::from("day_11_example_1.txt", rule)?;
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
        assert_eq!(solve_1("day_11_example_1.txt").unwrap(), "37");
    }

    #[test]
    fn test_solve_2() {
        let mut count = 0;
        for puzzle in GameOfSeats::from("day_11_example_1.txt", Puzzle2).unwrap().iter() {
            count += 1;
            println!("{}", count);
            println!("{}", puzzle);
            if count > 7 {break;}
        }
        println!("final count: {}", count);
        assert_eq!(solve_2("day_11_example_1.txt").unwrap(), "26");
    }

    #[test]
    fn test_solve_2_corners() {
        let mut gos = GameOfSeats::from("day_11_example_1.txt", Puzzle2).unwrap();
        let c = Position {row: 0, col: 0};
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), Seat::Floor);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 0);
        assert_eq!(gos.iterated_seat_at(c), Seat::Occupied);
        gos = gos.iterate();
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), Seat::Floor);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 3);
        assert_eq!(gos.iterated_seat_at(c), Seat::Occupied);
        gos = gos.iterate();
        assert_eq!(Puzzle2.projected_seat(&gos, Dir::N, c), Seat::Floor);
        assert_eq!(gos.num_adj_occupied(c).unwrap(), 1);
        assert_eq!(gos.seat(c), Seat::Occupied);
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