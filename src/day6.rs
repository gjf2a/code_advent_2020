use std::io;
use advent_code_lib::{MultiLineObjects, ExNihilo};
use std::collections::BTreeSet;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let answers = MultiLineObjects::from_file
        (filename,
        &mut |set: &mut BTreeSet<char>, line| {
            for c in line.chars() {
                set.insert(c);
            }
        })?;
    Ok(answers.iter().map(|s| s.len()).sum::<usize>().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let answers = MultiLineObjects::from_file
        (filename,
        &mut |p2g: &mut Puzzle2Group, line| {
            p2g.apply_line(line);
        })?;
    Ok(answers.iter().map(|p2g| p2g.num_selected()).sum::<usize>().to_string())
}

#[derive(Clone,Eq,PartialEq)]
struct Puzzle2Group {
    selected_chars: BTreeSet<char>
}

fn i2letter(i: u8) -> char {
    (i + 'a' as u8) as char
}

impl ExNihilo for Puzzle2Group {
    fn create() -> Self {
        let mut chars = BTreeSet::new();
        for i in 0..26 {
            chars.insert(i2letter(i));
        }
        Puzzle2Group {selected_chars: chars}
    }
}

impl Puzzle2Group {
    pub fn apply_line(&mut self, line: &str) {
        let absent: Vec<char> = self.selected_chars.iter()
            .filter(|c| !line.contains(**c))
            .map(|c| *c)
            .collect();

        for c in absent {
            self.selected_chars.remove(&c);
        }
    }

    pub fn num_selected(&self) -> usize {self.selected_chars.len()}

    pub fn chars_selected(&self) -> String {
        self.selected_chars.iter().collect()
    }
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
        let mut p2 = Puzzle2Group::create();
        p2.apply_line("abc");
        assert_eq!(p2.num_selected(), 3);
        assert_eq!(p2.chars_selected(), "abc");
        p2.apply_line("bcd");
        assert_eq!(p2.num_selected(), 2);
        assert_eq!(p2.chars_selected(), "bc");
        p2.apply_line("cde");
        assert_eq!(p2.num_selected(), 1);
        assert_eq!(p2.chars_selected(), "c");
        p2.apply_line("def");
        assert_eq!(p2.num_selected(), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2("day_6_example.txt").unwrap(), "6");
    }
}