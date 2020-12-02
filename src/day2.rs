use std::{io, fs};
use std::io::BufRead;

pub fn solve_1() -> io::Result<String> {
    let mut total = 0;
    for line in io::BufReader::new(fs::File::open("day_2_input.txt")?).lines() {
        if line_valid_password(line?.as_str()) {
            total += 1;
        }
    }
    Ok(format!("{}", total))
}

fn line_valid_password(line: &str) -> bool {
    let (lo, hi, letter, password) = parse_password_line(line);
    let count = password.matches(letter).count();
    count >= lo && count <= hi
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
    fn test_one_line() {
        assert!(line_valid_password("1-3 a: abcde"));
        assert!(!line_valid_password("1-3 b: cdefg"));
        assert!(line_valid_password("2-9 c: ccccccccc"));
        assert!(!line_valid_password("2-9 c: cccccccccc"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1().unwrap(), "550");
    }
}
