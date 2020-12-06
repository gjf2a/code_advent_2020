use std::io;
use advent_code_lib::for_each_line;

fn is_tree(c: u8) -> bool {
    c == '#' as u8
}

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(format!("{}", solve_slope(filename, 3, 1)?))
}

struct SledState {
    x: usize, y: usize, tree_count: usize, right: usize, down: usize
}

impl SledState {
    pub fn new(right: usize, down: usize) -> Self {
        SledState {x: 0, y: 0, tree_count: 0, right, down}
    }

    pub fn update(&mut self, line: &str) {
        let line = line.as_bytes();
        if self.y % self.down == 0 {
            if self.y > 0 && is_tree(line[self.x % line.len()]) {
                self.tree_count += 1;
            }
            self.x += self.right;
        }
        self.y += 1;
    }
}

fn solve_slope(filename: &str, right: usize, down: usize) -> io::Result<usize> {
    let mut sled = SledState::new(right, down);
    for_each_line(filename, |line| Ok({
        sled.update(line);
    }))?;
    Ok(sled.tree_count)
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