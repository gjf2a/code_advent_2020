use std::{io, mem};
use advent_code_lib::{all_lines, DirIter, Dir, Position};
use std::fmt::{Display, Formatter, Error};

const OCCUPIED: char = '#';
const FLOOR: char = '.';
const EMPTY: char = 'L';

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Puzzle1::from(filename)?.stable_state().num_occupied().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(Puzzle2::from(filename)?.stable_state().num_occupied().to_string())
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Puzzle1 {
    seating: Vec<Vec<char>>
}

impl Puzzle1 {
    fn from(filename: &str) -> io::Result<Self> {
        Ok(Puzzle1 {seating: Puzzle1::seating_from(filename)?})
    }
}

impl Display for Puzzle1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        format_seating(f, self.seating())
    }
}

impl GameOfSeatsPuzzle for Puzzle1 {
    fn seating(&self) -> &Vec<Vec<char>> {
        &self.seating
    }

    fn create_next(&self) -> Self {
        Puzzle1 {seating: self.create_next_seating()}
    }

    fn too_many_adj(&self) -> usize {
        4
    }

    fn project(&self, d: Dir, p: Position) -> Position {
        p.updated(d)
    }

    fn iter(&self) -> GameOfSeatsIterator<Self> {
        GameOfSeatsIterator { gos: Some(self.clone()) }
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Puzzle2 {
    seating: Vec<Vec<char>>
}

impl Puzzle2 {
    fn from(filename: &str) -> io::Result<Self> {
        Ok(Puzzle2 {seating: Puzzle2::seating_from(filename)?})
    }
}

impl GameOfSeatsPuzzle for Puzzle2 {
    fn seating(&self) -> &Vec<Vec<char>> {
        &self.seating
    }

    fn create_next(&self) -> Self {
        Puzzle2 {seating: self.create_next_seating()}
    }

    fn too_many_adj(&self) -> usize {
        5
    }

    fn project(&self, d: Dir, p: Position) -> Position {
        let mut p = p;
        loop {
            p.update(d);
            if !self.within_outer_ring(p) || self.seat(p) != FLOOR {
                return p;
            }
        }
    }

    fn iter(&self) -> GameOfSeatsIterator<Self> {
        GameOfSeatsIterator { gos: Some(self.clone()) }
    }
}

impl Display for Puzzle2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        format_seating(f, self.seating())
    }
}

fn format_seating(f: &mut Formatter<'_>, seating: &Vec<Vec<char>>) -> Result<(), Error> {
    write!(f, "{}", seating.iter()
        .map(|line| format!("{}\n", line.iter().collect::<String>()))
        .collect::<String>())
}

pub trait GameOfSeatsPuzzle : Sized + Eq {
    fn seating(&self) -> &Vec<Vec<char>>;
    fn create_next(&self) -> Self;
    fn too_many_adj(&self) -> usize;
    fn project(&self, d: Dir, p: Position) -> Position;
    fn iter(&self) -> GameOfSeatsIterator<Self>;

    fn seating_from(filename: &str) -> io::Result<Vec<Vec<char>>> {
        Ok(all_lines(filename)?
            .map(|line| line.chars().collect())
            .collect())
    }

    fn height(&self) -> usize {self.seating().len()}
    fn width(&self) -> usize {self.seating()[0].len()}

    fn seat(&self, p: Position) -> char {
        if self.in_bounds(p) {
            self.seating()[p.row as usize][p.col as usize]
        } else {
            FLOOR
        }
    }

    fn within_outer_ring(&self, p: Position) -> bool {
        p.col >= -1 && p.row >= -1 && p.col <= self.width() as isize && p.row <= self.height() as isize
    }

    fn in_bounds(&self, p: Position) -> bool {
        p.col >= 0 && p.row >= 0 && self.in_bounds_u(p.col as usize, p.row as usize)
    }

    fn in_bounds_u(&self, col: usize, row: usize) -> bool {
        col < self.width() && row < self.height()
    }

    fn num_adj_occupied(&self, p: Position) -> Option<usize> {
        if self.in_bounds(p) {
            Some(DirIter::new()
                .filter(|d| self.seat(self.project(*d, p)) == OCCUPIED)
                .count())
        } else {
            None
        }
    }

    fn create_next_seating(&self) -> Vec<Vec<char>> {
        (0..self.height())
                .map(|row| (0..self.width())
                    .map(|col| self.iterated_seat_at(Position {col: col as isize, row: row as isize}))
                    .collect())
                .collect()
    }

    fn iterated_seat_at(&self, p: Position) -> char {
        let seat = self.seat(p);
        let adj = self.num_adj_occupied(p).unwrap();
        if seat == EMPTY && adj == 0 {OCCUPIED}
        else if seat == OCCUPIED && adj >= self.too_many_adj() {EMPTY}
        else { seat }
    }

    fn stable_state(&self) -> Self {
        self.iter().last().unwrap()
    }

    fn num_occupied(&self) -> usize {
        self.seating().iter()
            .map(|row| row.iter()
                .filter(|s| **s == OCCUPIED)
                .count())
            .sum()
    }
}

pub struct GameOfSeatsIterator<T> {
    gos: Option<T>
}

impl <T:GameOfSeatsPuzzle+Eq> GameOfSeatsIterator<T> {
    fn update_next(&mut self, candidate: T) -> Option<<GameOfSeatsIterator<T> as Iterator>::Item> {
        let mut next = Some(candidate);
        if next == self.gos {
            self.gos = None;
        } else {
            mem::swap(&mut next, &mut self.gos);
        }
        next
    }
}

impl <T:GameOfSeatsPuzzle+Eq> Iterator for GameOfSeatsIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(gos) = &self.gos {
            let candidate = gos.create_next();
            self.update_next(candidate)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let start = Puzzle1::from("in/day11_ex1.txt").unwrap();
        assert_eq!(start.to_string(), EXPECTED_1[0]);
    }

    #[test]
    fn test_example_1() -> io::Result<()> {
        Ok(test_example(Puzzle1::from("in/day11_ex1.txt")?, &EXPECTED_1))
    }

    #[test]
    fn test_example_2() -> io::Result<()> {
        Ok(test_example(Puzzle2::from("in/day11_ex1.txt")?, &EXPECTED_2))
    }

    fn test_example<P:GameOfSeatsPuzzle+Display>(start: P, targets: &[&str]) {
        let mut iter = start.iter();
        for i in 0..targets.len() {
            assert_eq!(iter.next().unwrap().to_string(), targets[i]);
        }
        assert!(iter.next() == None);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("in/day11_ex1.txt").unwrap(), "37");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("in/day11_ex1.txt").unwrap(), "26");
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