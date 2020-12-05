use std::io;
use crate::all_lines;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::iter::Map;

fn seat_ids() -> io::Result<Map<Lines<BufReader<File>>, fn(io::Result<String>) -> usize>> {
    Ok(all_lines("day_5_input.txt")?
        .map(|line|
            BoardingPass::from(line.unwrap().as_str()).unwrap().seat_id()))
}

pub fn solve_1() -> io::Result<String> {
    Ok(seat_ids()?.max().unwrap().to_string())
}

pub fn solve_2() -> io::Result<String> {
    let mut ids: Vec<usize> = seat_ids()?.collect();
    ids.sort();
    for i in 1..ids.len() {
        if ids[i-1] + 2 == ids[i] {
            return Ok((ids[i-1] + 1).to_string())
        }
    }
    panic!("This shouldn't happen; the seat isn't there")
}

fn decode_str(encoding: &str, lo: char, hi: char) -> Option<usize> {
    let encoding = encoding.as_bytes();
    let lo = lo as u8;
    let hi = hi as u8;
    let mut min = 0;
    let mut max = (1 << encoding.len()) - 1;
    for code in encoding.iter() {
        let mid = (min + max) / 2;
        if *code == lo {
            max = mid;
        } else if *code == hi {
            min = mid + 1;
        } else {
            return None
        }
    }
    assert_eq!(min, max);
    Some(min)
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub struct BoardingPass {
    row: usize,
    col: usize
}

impl BoardingPass {
    pub fn from(encoding: &str) -> Option<Self> {
        if encoding.len() == 10 {
            Some(BoardingPass {
                row: decode_str(&encoding[0..7], 'F', 'B').unwrap(),
                col: decode_str(&encoding[7..], 'L', 'R').unwrap() })
        } else {None}
    }

    pub fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }

    pub fn row(&self) -> usize {self.row}

    pub fn col(&self) -> usize {self.col}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let pass = BoardingPass::from("FBFBBFFRLR").unwrap();
        assert_eq!(pass.row(), 44);
        assert_eq!(pass.col(), 5);
        assert_eq!(pass.seat_id(), 357);
    }

    #[test]
    fn test_decode_1() {
        let pass = BoardingPass::from("BFFFBBFRRR").unwrap();
        assert_eq!(pass.row(), 70);
        assert_eq!(pass.col(), 7);
        assert_eq!(pass.seat_id(), 567);
    }

    #[test]
    fn test_decode_2() {
        let pass = BoardingPass::from("FFFBBBFRRR").unwrap();
        assert_eq!(pass.row(), 14);
        assert_eq!(pass.col(), 7);
        assert_eq!(pass.seat_id(), 119);
    }

    #[test]
    fn test_decode_3() {
        let pass = BoardingPass::from("BBFFBBFRLL").unwrap();
        assert_eq!(pass.row(), 102);
        assert_eq!(pass.col(), 4);
        assert_eq!(pass.seat_id(), 820);
    }
}