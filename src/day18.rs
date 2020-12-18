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
/*
struct Evaluator {
    total: usize,
    last_op: char,
    done: bool,
    mul_func:
}

impl Evaluator {
    pub fn eval<T:Iterator<Item=char>>(chars: &mut T) -> usize {
        let mut eval = Evaluator { total: 0, last_op: '+', done: false };
        while !eval.done {
            eval.update(chars);
        }
        eval.total
    }

    pub fn update_total(&mut self, value: usize) {
        self.total = op(self.total, self.last_op, value);
    }

    pub fn update<T:Iterator<Item=char>>(&mut self, chars: &mut T) {
        let c = chars.next();
        if let Some(c) = c {
            match c {
                '*' | '+' => self.last_op = c,
                '0'..='9' => self.update_total(parse_digit(c)),
                ' ' => {},
                '(' => self.update_total(Evaluator::eval(chars)),
                ')' => {self.done = true},
                _ => panic!("Unrecognized input character: '{}'", c)
            }
        } else {
            self.done = true;
        }
    }
}

pub fn eval_1(line: &str) -> usize {
    Evaluator::eval(&mut line.chars())
}
*/

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
/*
pub fn eval<I:Iterator<Item=char>,F:Fn(&mut I, &F)->usize>(chars: &mut I, mult_next: &F) -> usize {
    //let mut chars = chars.filter(|c| *c != ' '); // Killed the compiler
    let mut total = grab_next_value(chars, &mult_next);
    loop {
        if let Some(c) = chars.next() {
            total = match c {
                '+' => total + grab_next_value(chars, mult_next),
                '*' => total * mult_next(chars, mult_next),
                ')' => return total,
                _ => grab_next_value(chars, mult_next)
            }
        } else {
            return total;
        }
    }
}

pub fn grab_next_value<I:Iterator<Item=char>,F:Fn(&mut I, F)->usize>(chars: &mut I, mult_next: &F) -> usize {
    let c = chars.next().unwrap();
    match c {
        '0'..='9' => parse_digit(c),
        '(' => eval(chars, &mult_next),
        _ => panic!("Unrecognized char: '{}'", c)
    }
}
*/
/*pub fn eval_1(line: &str) -> usize {
    eval_chars_1(&mut line.chars())
}*/

pub fn eval_chars_1(chars: &mut Chars) -> usize {
    let mut total = 0;
    let mut last_op = '+';
    loop {
        let c = chars.next();
        if let Some(c) = c {
            match c {
                '*' | '+' => last_op = c,
                '0'..='9' => total = op(total, last_op, parse_digit(c)),
                ' ' => {},
                '(' => total = op(total, last_op, eval_chars_1(chars)),
                ')' => return total,
                _ => panic!("Unrecognized input character: '{}'", c)
            }
        } else {
            return total;
        }
    }
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

fn op(total: usize, op: char, num: usize) -> usize {
    match op {
        '+' => total + num,
        '*' => total * num,
        _ => panic!("Unrecognized operator: '{}'", op)
    }
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