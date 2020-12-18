use std::io;
use advent_code_lib::all_lines;
use core::iter::Peekable;

pub fn solve(filename: &str, puzzle: Puzzle) -> io::Result<String> {
    Ok(all_lines(filename)?.map(|line| Evaluator::new(line.chars(), puzzle).eval()).sum::<usize>().to_string())
}

#[derive(Copy,Clone,Debug)]
pub enum Puzzle {
    One, Two
}

pub struct Evaluator<I:Iterator<Item=char>> {
    chars: Peekable<I>,
    puzzle: Puzzle
}

impl <I:Iterator<Item=char>> Evaluator<I> {
    pub fn new(chars: I, puzzle: Puzzle) -> Evaluator<I> {
        Evaluator {chars: chars.peekable(), puzzle}
    }

    pub fn eval(&mut self) -> usize {
        let mut total = self.grab_next_value();
        loop {
            let peeked = self.chars.peek();
            if peeked == None || *(peeked.unwrap()) == ')' {
                return total;
            } else {
                total = match self.chars.next().unwrap() {
                    '+' => total + self.grab_next_value(),
                    '*' => total * match self.puzzle {Puzzle::One => self.grab_next_value(), Puzzle::Two => self.eval()},
                    ' ' => total,
                    _ => panic!("This shouldn't happen")
                };
            }
        }
    }

    pub fn grab_next_value(&mut self) -> usize {
        let c = self.chars.next().unwrap();
        match c {
            '0'..='9' => parse_digit(c),
            '(' => {
                let result = self.eval();
                let next = self.chars.next().unwrap();
                assert_eq!(next, ')');
                result
            },
            ' ' => self.grab_next_value(),
            _ => panic!("Unrecognized char: '{}'", c)
        }
    }
}

pub fn eval_1(line: &str) -> usize {
    Evaluator::new(line.chars(), Puzzle::One).eval()
}

pub fn eval_2(line: &str) -> usize {
    Evaluator::new(line.chars(), Puzzle::Two).eval()
}

fn parse_digit(digit: char) -> usize {
    digit as usize - '0' as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_1() {
        for (line, target) in &[
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)
        ] {
            assert_eq!(eval_1(line), *target);
        }
    }

    #[test]
    fn test_eval_2() {
        for (line, target) in &[
            ("(5 * 2 + (3 * 2)) + 2", 42),
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("2 + 4 * 9", 54),
            ("6 + 9 * 8 + 6", 210),
            ("(2 + 4 * 9) * (6 + 9 * 8 + 6) + 6", 11664),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6)", 11664),
            ("(5 * 2 + 6) + 2", 42),
            ("(5 * 2 + (3 * 2))", 40),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2", 11666),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4", 11670),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)
        ] {
            println!("evaluating {}; target {}", line, target);
            assert_eq!(eval_2(line), *target);
        }
    }
}