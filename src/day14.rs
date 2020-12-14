use std::io;
use advent_code_lib::all_lines;
use std::collections::BTreeMap;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let lines = all_lines(filename)?.map(|line| line.unwrap());
    let mut mask = Mask1 {on: 0, off: 0};
    let mut mem = BTreeMap::new();
    lines.for_each(|line| {
        if line.starts_with("mask") {
            mask = Mask1::from(line.as_str());
        } else {
            let (idx, val) = split_mem(line.as_str());
            mem.insert(idx, mask.mask(val));
        }
    });
    let mem_sum: u64 = mem.values().sum();
    Ok(mem_sum.to_string())
}

fn split_mem(line: &str) -> (usize, u64) {
    let tokens: Vec<_> = line.split(&['[', ']', '=', ' '][..]).collect();
    let idx = tokens[1].parse::<usize>().unwrap();
    let val = tokens[5].parse::<u64>().unwrap();
    (idx, val)
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct Mask1 {
    on: u64,
    off: u64
}

impl Mask1 {
    pub fn from(line: &str) -> Self {
        let mut m = Mask1 {on: 0, off: 0};
        line.chars().skip_while(|c| "mask = ".contains(*c))
            .for_each(|c| {
            m.on <<= 1;
            m.off <<= 1;
            match c {
                'X' => {m.off |= 1;},
                '0' => {},
                '1' => {m.on |= 1; m.off |= 1;},
                _ => panic!("Error! char '{}' unknown", c)
            }
        });
        m
    }

    pub fn mask(&self, value: u64) -> u64 {
        value & self.off | self.on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ones(num_ones: u32) -> u64 {
        2_u64.pow(num_ones) - 1
    }

    #[test]
    fn test_1() {
        assert_eq!(solve_1("day_14_ex.txt").unwrap(), "165");
    }

    #[test]
    fn test_mask_1() {
        let m_target = Mask1 {on: 64, off: !2_u64 & make_ones(36)};
        let m_created = Mask1::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(m_created, m_target);
    }

    #[test]
    fn test_num_ones() {
        [(2, 3), (3, 7), (4, 15), (5, 31), (6, 63), (7, 127), (8, 255), (9, 511), (10, 1023)]
            .iter().for_each(|(num, target)| {
            assert_eq!(make_ones(*num), *target);
        });
    }
}