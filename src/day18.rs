use std::io;
use advent_code_lib::all_lines;
use core::iter::Peekable;
use std::str::Chars;

fn puzzle_1(line: &str) -> Evaluator<Chars> {
    Evaluator::new(line.chars(), Evaluator::grab_next_value)
}

fn puzzle_2(line: &str) -> Evaluator<Chars> {
    Evaluator::new(line.chars(), Evaluator::eval)
}

pub fn solve_1(filename: &str) -> io::Result<String> {
    solve(filename, puzzle_1)
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    solve(filename, puzzle_2)
}

pub fn solve(filename: &str, puzzle: fn(&str)->Evaluator<Chars>) -> io::Result<String> {
    Ok(all_lines(filename)?
        .map(|line| puzzle(line.as_str()).eval())
        .sum::<usize>().to_string())
}

pub struct Evaluator<I:Iterator<Item=char>> {
    chars: Peekable<I>,
    puzzle: fn(&mut Evaluator<I>) -> usize
}

impl <I:Iterator<Item=char>> Evaluator<I> {
    pub fn new(chars: I, puzzle: fn(&mut Evaluator<I>) -> usize) -> Evaluator<I> {
        Evaluator {chars: chars.peekable(), puzzle}
    }

    pub fn eval(&mut self) -> usize {
        let mut total = self.grab_next_value();
        while !self.at_expr_end() {
            total = match self.chars.next().unwrap() {
                '+' => total + self.grab_next_value(),
                '*' => total * (self.puzzle)(self),
                ' ' => total,
                _ => panic!("This shouldn't happen")
            };
        }
        total
    }

    pub fn at_expr_end(&mut self) -> bool {
        let peeked = self.chars.peek();
        peeked == None || *(peeked.unwrap()) == ')'
    }

    pub fn grab_next_value(&mut self) -> usize {
        let c = self.chars.next().unwrap();
        match c {
            '0'..='9' => parse_digit(c),
            '(' => self.parse_check_paren(),
            ' ' => self.grab_next_value(),
            _ => panic!("Unrecognized char: '{}'", c)
        }
    }

    fn parse_check_paren(&mut self) -> usize {
        let result = self.eval();
        let next = self.chars.next().unwrap();
        assert_eq!(next, ')');
        result
    }
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
            assert_eq!(puzzle_1(line).eval(), *target);
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
            assert_eq!(puzzle_2(line).eval(), *target);
        }
    }
}