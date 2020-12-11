use std::io;
use advent_code_lib::{all_lines, DirIter};
use std::fmt::{Display, Formatter, Error};

pub fn solve_1(filename: &str) -> io::Result<String> {
    let gos = GameOfSeats::from(filename, 4)?;
    Ok(gos.stable_state().num_occupied().to_string())
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

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct GameOfSeats {
    seating: Vec<Vec<Seat>>,
    max_adjacent: usize
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
    pub fn from(filename: &str, max_adjacent: usize) -> io::Result<Self> {
        Ok(GameOfSeats {
            seating: all_lines(filename)?
                .map(|line| line.unwrap().chars()
                    .map(|c| Seat::from(c).unwrap())
                    .collect())
                .collect(),
            max_adjacent,
        })
    }

    pub fn height(&self) -> usize {self.seating.len()}

    pub fn width(&self) -> usize {self.seating[0].len()}

    pub fn seat_i(&self, col: isize, row: isize) -> Seat {
        if self.in_bounds_i(col, row) {
            self.seating[row as usize][col as usize]
        } else {
            Seat::Floor
        }
    }

    pub fn seat(&self, col: usize, row: usize) -> Seat {
        self.seat_i(col as isize, row as isize)
    }

    pub fn in_bounds_i(&self, col: isize, row: isize) -> bool {
        col >= 0 && row >= 0 && self.in_bounds_u(col as usize, row as usize)
    }

    pub fn in_bounds_u(&self, col: usize, row: usize) -> bool {
        col < self.width() && row < self.height()
    }

    pub fn num_adj_occupied(&self, col: usize, row: usize) -> Option<usize> {
        if self.in_bounds_u(col, row) {
            Some(DirIter::new()
                .map(|d| d.neighbor(col, row))
                .filter(|(col, row)| self.seat_i(*col, *row) == Seat::Occupied)
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
                    .map(|col| {
                        let seat = self.seat(col, row);
                        let adj = self.num_adj_occupied(col, row).unwrap();
                        if seat == Seat::Empty && adj == 0 {
                            Seat::Occupied
                        } else if seat == Seat::Occupied && adj >= self.max_adjacent {
                            Seat::Empty
                        } else { seat }
                    })
                    .collect())
                .collect(),
            max_adjacent: self.max_adjacent
        }
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
        let result = self.gos.clone();
        self.gos = match &self.gos {
            None => None,
            Some(gos) => {
                let candidate = gos.iterate();
                if candidate == *gos {
                    None
                } else {
                    Some(candidate)
                }
            }
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: [&'static str; 6] = [
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

    #[test]
    fn test_create() {
        let start = GameOfSeats::from("day_11_example_1.txt", 4).unwrap();
        assert_eq!(start.to_string(), EXPECTED[0]);
    }

    #[test]
    fn test_example_1() -> io::Result<()> {
        let start = GameOfSeats::from("day_11_example_1.txt", 4)?;
        let mut iter = start.iter();
        for i in 0..EXPECTED.len() {
            assert_eq!(iter.next().unwrap().to_string(), EXPECTED[i]);
        }
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("day_11_example_1.txt").unwrap(), "37");
    }
}