use std::collections::btree_set::BTreeSet;
use std::io;
use crate::MultiLineObjects;
use bits::BitArray;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let answers = MultiLineObjects::from_file
        (Box::new(BTreeSet::new),
         filename,
        &mut |set, line| {
            for c in line.chars() {
                set.insert(c);
            }
        })?;
    Ok(answers.iter().map(|s| s.len()).sum::<usize>().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let answers = MultiLineObjects::from_file
        (Box::new(Puzzle2Group::new),
        filename,
        &mut |p2g, line| {
            p2g.apply_line(line);
        })?;
    Ok(answers.iter().map(|p2g| p2g.num_selected()).sum::<u32>().to_string())
}

#[derive(Clone,Eq,PartialEq)]
struct Puzzle2Group {
    selected_chars: BitArray
}

impl Puzzle2Group {
    pub fn new() -> Self {
        let mut bits = BitArray::new();
        for _ in 0..26 {
            bits.add(true);
        }
        Puzzle2Group {selected_chars: bits}
    }

    pub fn apply_line(&mut self, line: &str) {
        for i in 0..26 {
            let c = (i + 'a' as u8) as char;
            if !line.contains(c) {
                self.selected_chars.set(i as u64, false);
            }
        }
    }

    pub fn num_selected(&self) -> u32 {self.selected_chars.count_bits_on()}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1("day_6_example.txt").unwrap(), "11");
    }

    #[test]
    fn test_p2_group() {
        let mut p2 = Puzzle2Group::new();
        p2.apply_line("abc");
        assert_eq!(p2.num_selected(), 3);
        p2.apply_line("bcd");
        assert_eq!(p2.num_selected(), 2);
        p2.apply_line("cde");
        assert_eq!(p2.num_selected(), 1);
        p2.apply_line("def");
        assert_eq!(p2.num_selected(), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2("day_6_example.txt").unwrap(), "6");
    }
}