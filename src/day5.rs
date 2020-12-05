use std::io;
use crate::all_lines;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::iter::Map;

fn seat_ids() -> io::Result<Map<Lines<BufReader<File>>, fn(io::Result<String>) -> usize>> {
    Ok(all_lines("day_5_input.txt")?
        .map(|line| BoardingPass::from(line.unwrap().as_str()).seat_id()))
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

fn decode_str(encoding: &str, lo: char, hi: char) -> usize {
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
            panic!("Illegal character: {}", *code as char);
        }
    }
    assert_eq!(min, max);
    min
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub struct BoardingPass {
    row: usize,
    col: usize
}

impl BoardingPass {
    pub fn from(encoding: &str) -> Self {
        if encoding.len() == 10 {
            BoardingPass {
                row: decode_str(&encoding[..7], 'F', 'B'),
                col: decode_str(&encoding[7..], 'L', 'R') }
        } else {panic!("Illegal encoding length {}: {}", encoding.len(), encoding)}
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

    fn test(encoding: &str, row: usize, col: usize, id: usize) {
        let pass = BoardingPass::from(encoding);
        assert_eq!(pass.row(), row);
        assert_eq!(pass.col(), col);
        assert_eq!(pass.seat_id(), id);
    }

    #[test]
    fn test_decode() {
        test("FBFBBFFRLR", 44, 5, 357);
    }

    #[test]
    fn test_decode_1() {
        test("BFFFBBFRRR", 70, 7, 567);
    }

    #[test]
    fn test_decode_2() {
        test("FFFBBBFRRR", 14, 7, 119);
    }

    #[test]
    fn test_decode_3() {
        test("BBFFBBFRLL", 102, 4, 820);
    }
}