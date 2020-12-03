use std::io;
use crate::for_each_line;

fn is_tree(c: u8) -> bool {
    c == '#' as u8
}

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(format!("{}", solve_slope(filename, 3, 1)?))
}

fn solve_slope(filename: &str, right: usize, down: usize) -> io::Result<usize> {
    let mut x_pos = 0;
    let mut tree_count = 0;
    let mut y_pos = 0;
    for_each_line(filename, |line| Ok({
        let line = line.as_bytes();
        if y_pos % down == 0 {
            if y_pos > 0 && is_tree(line[x_pos % line.len()]) {
                tree_count += 1;
            }
            x_pos += right;
        }
        y_pos += 1;
    }))?;
    Ok(tree_count)
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(format!("{}", [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|(r, d)| solve_slope(filename, *r, *d).unwrap())
        .product::<usize>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_1("day_3_example.txt").unwrap(), "7");
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_2("day_3_example.txt").unwrap(), "336")
    }

    #[test]
    fn example_3() {
        assert_eq!(solve_slope("day_3_example.txt", 1, 2).unwrap(), 2)
    }
}