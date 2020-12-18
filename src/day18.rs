use std::str::Chars;
use std::io;
use advent_code_lib::all_lines;
use std::iter::Rev;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(all_lines(filename)?.map(|line| eval_1(line.as_str())).sum::<usize>().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(all_lines(filename)?.map(|line| eval_2(line.as_str())).sum::<usize>().to_string())
}

pub enum Puzzle {
    One, Two
}

pub struct Evaluator<I> {
    chars: I,
    puzzle: Puzzle
}

impl <I:Iterator<Item=char>> Evaluator<I> {
    pub fn eval(&mut self) -> usize {
        let mut total = self.grab_next_value();
        loop {
            if let Some(c) = self.chars.next() {
                total = match c {
                    '+' => total + self.grab_next_value(),
                    '*' => total * match self.puzzle {Puzzle::One => self.grab_next_value(), Puzzle::Two => self.eval()},
                    ')' => return total,
                    _ => self.grab_next_value()
                }
            } else {
                return total;
            }
        }
    }

    pub fn grab_next_value(&mut self) -> usize {
        let c = self.chars.next().unwrap();
        match c {
            '0'..='9' => parse_digit(c),
            '(' => self.eval(),
            _ => panic!("Unrecognized char: '{}'", c)
        }
    }
}

pub fn eval_1(line: &str) -> usize {
    Evaluator {chars: line.chars().filter(|c| *c != ' '), puzzle: Puzzle::One}.eval()
}

pub fn eval_2(line: &str) -> usize {
    eval_chars_2(&mut line.chars().rev())
}

fn eval_chars_2(chars: &mut Rev<Chars>) -> usize {
    let mut last_val = 1;
    loop {
        let c = chars.next();
        if let Some(c) = c {
            match c {
                '0'..='9' => last_val = parse_digit(c) * last_val,
                '(' => last_val = eval_chars_2(chars),
                ' ' | '*' => {},
                '+' => last_val = last_val + eval_chars_2(chars),
                ')' => return last_val,
                _ => panic!("Unrecognized input character: '{}'", c)
            }
        } else {
            return last_val;
        }
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
            assert_eq!(eval_1(line), *target);
        }
    }

    #[test]
    fn test_eval_2() {
        for (line, target) in &[
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)
        ] {
            assert_eq!(eval_2(line), *target);
        }
    }
}