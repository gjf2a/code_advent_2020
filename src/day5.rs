use std::io;
use advent_code_lib::all_lines;
use std::fmt::Display;

fn seat_ids() -> io::Result<impl Iterator<Item=usize>> {
    Ok(all_lines("in/day5.txt")?
        .map(|line| BoardingPass::from(line.as_str()).seat_id()))
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

struct BinarySearcher<T:Eq+Copy+Display> {
    min: usize, max: usize, lo: T, hi: T
}

impl <T:Eq+Copy+Display> BinarySearcher<T> {
    pub fn test(&mut self, test_val: T) {
        let mid = (self.min + self.max) / 2;
        if test_val == self.lo {
            self.max = mid;
        } else if test_val == self.hi {
            self.min = mid + 1;
        } else {
            panic!("Illegal value: {}", test_val);
        }
    }

    pub fn value(&self) -> usize {
        assert!(self.min == self.max);
        self.min
    }
}

fn decode_str(encoding: &str, lo: char, hi: char) -> usize {
    let encoding = encoding.as_bytes();
    let mut searcher = BinarySearcher  {min: 0, max: (1 << encoding.len()) - 1, lo: lo as u8, hi: hi as u8};
    for code in encoding.iter() {
        searcher.test(*code);
    }
    searcher.value()
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