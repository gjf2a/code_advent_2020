use std::io;
use advent_code_lib::all_lines;
use std::collections::BTreeMap;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Mask1::from("").solve(filename)
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Mask2::from("").solve(filename)
}

pub trait Solver {
    fn update_mask(&mut self, line: &str);
    fn update_mem(&self, idx: u64, val: u64, mem: &mut BTreeMap<u64,u64>);

    fn solve(&mut self, filename: &str) -> io::Result<String> {
        let lines = all_lines(filename)?.map(|line| line.unwrap());
        let mut mem = BTreeMap::new();
        lines.for_each(|line| {
            if line.starts_with("mask") {
                self.update_mask(line.as_str());
            } else {
                let (idx, val) = split_mem(line.as_str());
                self.update_mem(idx, val, &mut mem);
            }
        });
        let mem_sum: u64 = mem.values().sum();
        Ok(mem_sum.to_string())
    }
}

fn split_mem(line: &str) -> (u64, u64) {
    let tokens: Vec<_> = line.split(&['[', ']', '=', ' '][..]).collect();
    let idx = tokens[1].parse::<u64>().unwrap();
    let val = tokens[5].parse::<u64>().unwrap();
    (idx, val)
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct Mask1 {
    on: u64,
    off: u64
}

impl Solver for Mask1 {
    fn update_mask(&mut self, line: &str) {
        self.replace_from(line);
    }

    fn update_mem(&self, idx: u64, val: u64, mem: &mut BTreeMap<u64, u64>) {
        mem.insert(idx, self.mask(val));
    }
}

impl Mask1 {
    pub fn from(line: &str) -> Self {
        let mut m = Mask1 { on: 0, off: 0 };
        m.replace_from(line);
        m
    }

    pub fn replace_from(&mut self, line: &str) {
        self.on = 0;
        self.off = 0;
        line.chars().skip_while(|c| "mask = ".contains(*c))
            .for_each(|c| self.add(c));
    }

    pub fn add(&mut self, c: char) {
        self.on <<= 1;
        self.off <<= 1;
        match c {
            'X' => { self.off |= 1; },
            '0' => {},
            '1' => {
                self.on |= 1;
                self.off |= 1;
            },
            _ => panic!("Error! char '{}' unknown", c)
        }
    }

    pub fn mask(&self, value: u64) -> u64 {
        value & self.off | self.on
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Mask2 {
    versions: Vec<Mask1>
}

impl Solver for Mask2 {
    fn update_mask(&mut self, line: &str) {
        self.replace_from(line);
    }

    fn update_mem(&self, idx: u64, val: u64, mem: &mut BTreeMap<u64, u64>) {
        for option in self.all_variants_of(idx) {
            mem.insert(option, val);
        }
    }
}

impl Mask2 {
    pub fn from(line: &str) -> Self {
        let mut mask = Mask2 {versions: Vec::new()};
        mask.replace_from(line);
        mask
    }

    pub fn replace_from(&mut self, line: &str) {
        self.versions = vec![Mask1::from("")];
        line.chars().skip_while(|c| "mask = ".contains(*c))
            .for_each(|c| {
                match c {
                    '0' => self.add_to_all('X'),
                    '1' => self.add_to_all('1'),
                    'X' => {
                        let mut copy = self.versions.clone();
                        add_to_all(&mut copy, '0');
                        self.add_to_all('1');
                        self.versions.append(&mut copy);
                    }
                    _ => panic!("Error! char '{}' unknown", c)
                }
            });
    }

    fn add_to_all(&mut self, c: char) {
        add_to_all(&mut self.versions, c);
    }

    pub fn all_variants_of(&self, value: u64) -> Vec<u64> {
        self.versions.iter().map(|m| m.mask(value)).collect()
    }
}

fn add_to_all(masks: &mut Vec<Mask1>, c: char) {
    masks.iter_mut().for_each(|m| m.add(c));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ones(num_ones: u32) -> u64 {
        2_u64.pow(num_ones) - 1
    }

    #[test]
    fn test_1() {
        assert_eq!(solve_1("in/day14_ex.txt").unwrap(), "165");
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

    #[test]
    fn test_mask_2_1() {
        for (mask, value, target) in
            &[("000000000000000000000000000000X1001X", 42, vec![26, 27, 58, 59]),
              ("00000000000000000000000000000000X0XX", 26, vec![16, 17, 18, 19, 24, 25, 26, 27])] {
            let mask = Mask2::from(mask);
            let mut variants = mask.all_variants_of(*value);
            variants.sort();
            assert_eq!(variants, *target);
        }
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("in/day14.txt").unwrap(), "17481577045893");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("in/day14.txt").unwrap(), "4160009892257");
    }
}