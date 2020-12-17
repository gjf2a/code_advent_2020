use std::{io, mem};
use advent_code_lib::{all_lines, DirIter, Dir, Position};
use std::fmt::{Display, Formatter, Error};

const OCCUPIED: char = '#';
const FLOOR: char = '.';
const EMPTY: char = 'L';

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(num_occupied_at_stable(puzzle_1_iter(GameOfSeats::from(filename)?)).to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(num_occupied_at_stable(puzzle_2_iter(GameOfSeats::from(filename)?)).to_string())
}

pub fn num_occupied_at_stable(iter: GameOfSeatsIterator) -> usize {
    iter.last().unwrap().num_occupied()
}

pub fn puzzle_1_iter(start: GameOfSeats) -> GameOfSeatsIterator {
    GameOfSeatsIterator { gos: Some(start), too_many_adj: 4, projection: |_,d,p| p.updated(d) }
}

pub fn puzzle_2_iter(start: GameOfSeats) -> GameOfSeatsIterator {
    GameOfSeatsIterator {
        gos: Some(start),
        too_many_adj: 5,
        projection: |gos, d, p| {
            let mut p = p;
            loop {
                p.update(d);
                if !gos.within_outer_ring(p) || gos.seat(p) != FLOOR {
                    return p;
                }
            }
        },
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct GameOfSeats {
    seating: Vec<Vec<char>>
}

impl Display for GameOfSeats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.seating.iter()
            .map(|line| format!("{}\n", line.iter().collect::<String>()))
            .collect::<String>())
    }
}

impl GameOfSeats {
    pub fn from(filename: &str) -> io::Result<Self> {
        Ok(GameOfSeats {
            seating: all_lines(filename)?
                .map(|line| line.chars().collect())
                .collect()
        })
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
        p.col >= 0 && p.row >= 0 && p.col < self.width() as isize && p.row < self.height() as isize
    }

    pub fn num_occupied(&self) -> usize {
        self.seating.iter()
            .map(|row| row.iter()
                .filter(|s| **s == OCCUPIED)
                .count())
            .sum()
    }
}

#[derive(Clone)]
pub struct GameOfSeatsIterator {
    gos: Option<GameOfSeats>,
    too_many_adj: usize,
    projection: fn(&GameOfSeats,Dir,Position)->Position
}

impl GameOfSeatsIterator {
    pub fn create_next(&self) -> Option<GameOfSeats> {
        if let Some(gos) = &self.gos {
            Some(GameOfSeats {
                seating: (0..gos.height())
                    .map(|row| (0..gos.width())
                        .map(|col| self.iterated_seat_at(Position { col: col as isize, row: row as isize }))
                        .collect())
                    .collect()
            })
        } else {
            None
        }
    }

    pub fn iterated_seat_at(&self, p: Position) -> char {
        let gos = &self.gos.as_ref().unwrap();
        let seat = gos.seat(p);
        let adj = self.num_adj_occupied(p);
        if seat == EMPTY && adj == 0 {OCCUPIED}
        else if seat == OCCUPIED && adj >= self.too_many_adj {EMPTY}
        else { seat }
    }

    pub fn num_adj_occupied(&self, p: Position) -> usize {
        let gos = &self.gos.as_ref().unwrap();
        if gos.in_bounds(p) {
            DirIter::new()
                .filter(|d| gos.seat((self.projection)(gos, *d, p)) == OCCUPIED)
                .count()
        } else {
            0
        }
    }
}

impl Iterator for GameOfSeatsIterator {
    type Item = GameOfSeats;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.create_next();
        if next == self.gos {
            self.gos = None;
        } else {
            mem::swap(&mut next, &mut self.gos);
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let start = GameOfSeats::from("in/day11_ex1.txt").unwrap();
        assert_eq!(start.to_string(), EXPECTED_1[0]);
    }

    #[test]
    fn test_example_1() -> io::Result<()> {
        test_example(puzzle_1_iter(GameOfSeats::from("in/day11_ex1.txt")?), &EXPECTED_1);
        Ok(())
    }

    #[test]
    fn test_example_2() -> io::Result<()> {
        test_example(puzzle_2_iter(GameOfSeats::from("in/day11_ex1.txt")?), &EXPECTED_2);
        Ok(())
    }

    fn test_example(mut iter: GameOfSeatsIterator, targets: &[&str]) {
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