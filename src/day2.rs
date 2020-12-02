use std::io;
use crate::pass_counter;

pub fn solve_1() -> io::Result<String> {
    pass_counter(line_valid_password_1)
}

pub fn solve_2() -> io::Result<String> {
    pass_counter(line_valid_password_2)
}

fn line_valid_password_1(line: &str) -> bool {
    let (lo, hi, letter, password) = parse_password_line(line);
    let count = password.matches(letter).count();
    count >= lo && count <= hi
}

fn line_valid_password_2(line: &str) -> bool {
    let (lo, hi, letter, password) = parse_password_line(line);
    let password = password.as_bytes();
    if lo > 0 && lo <= password.len() && hi > 0 && hi <= password.len() {
        let lo = lo - 1;
        let hi = hi - 1;
        let at_lo = password[lo] == letter as u8;
        let at_hi = password[hi] == letter as u8;
        at_lo != at_hi
    } else {
        false
    }
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
