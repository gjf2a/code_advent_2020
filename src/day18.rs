use std::str::Chars;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(all_lines(filename)?.map(|line| eval(line.as_str())).sum::<usize>().to_string())
}

pub fn eval(line: &str) -> usize {
    eval_chars(&mut line.chars())
}

pub fn eval_chars(chars: &mut Chars) -> usize {
    let mut total = 0;
    let mut last_op = '+';
    loop {
        let c = chars.next();
        if let Some(c) = c {
            match c {
                '*' | '+' => last_op = c,
                '0'..='9' => total = op(total, last_op, c as usize - '0' as usize),
                ' ' => {},
                '(' => total = op(total, last_op, eval_chars(chars)),
                ')' => return total,
                _ => panic!("Unrecognized input character: '{}'", c)
            }
        } else {
            return total;
        }
    }
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
    fn test_eval_line() {
        for (line, target) in &[
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)
        ] {
            assert_eq!(eval(line), *target);
        }
    }
}