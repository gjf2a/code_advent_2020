use std::io;
use crate::pass_counter;

pub fn solve_1() -> io::Result<String> {
    pass_counter("day_2_input.txt", line_valid_password_1)
}

pub fn solve_2() -> io::Result<String> {
    pass_counter("day_2_input.txt", line_valid_password_2)
}

fn line_valid_password_1(line: &str) -> bool {
    let (lo, hi, letter, password) = parse_password_line(line);
    let count = password.matches(letter).count();
    count >= lo && count <= hi
}

fn in_bounds(i: usize, password: &[u8]) -> bool {
    i > 0 && i <= password.len()
}

fn at(i: usize, password: &[u8], letter: char) -> bool {
    password[i - 1] == letter as u8
}

fn line_valid_password_2(line: &str) -> bool {
    let (lo, hi, letter, password) = parse_password_line(line);
    let password = password.as_bytes();
    in_bounds(lo, password) && in_bounds(hi, password) &&
        at(lo, password, letter) != at(hi, password, letter)
}

fn parse_password_line(line: &str) -> (usize,usize,char,&str) {
    let mut parts = line.split_whitespace();
    let mut range = parts.next().unwrap().split('-');
    let lo = range.next().unwrap().parse::<usize>().unwrap();
    let hi = range.next().unwrap().parse::<usize>().unwrap();
    (lo, hi, parts.next().unwrap().chars().next().unwrap(), parts.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_line_1() {
        assert!(line_valid_password_1("1-3 a: abcde"));
        assert!(!line_valid_password_1("1-3 b: cdefg"));
        assert!(line_valid_password_1("2-9 c: ccccccccc"));
        assert!(!line_valid_password_1("2-9 c: cccccccccc"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1().unwrap(), "550");
    }

    #[test]
    fn test_one_line_2() {
        assert!(line_valid_password_2("1-3 a: abcde"));
        assert!(!line_valid_password_2("1-3 b: cdefg"));
        assert!(!line_valid_password_2("2-9 c: ccccccccc"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_2().unwrap(), "634");
    }
}
