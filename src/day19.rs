use std::collections::BTreeMap;
use smallvec::SmallVec;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Rules::puzzle1(filename)?.to_string())
}

#[derive(Clone,Debug)]
enum Rule {
    Char(char), Subrules(SmallVec<[usize; 3]>), Alt(Box<Rule>,Box<Rule>),
    Eight, Eleven
}

struct Rules {
    rules: BTreeMap<usize,Rule>
}

fn decode_option(chars: &str) -> Rule {
    let chars = chars.trim();
    let chars_bytes = chars.as_bytes();
    if chars_bytes[0] == '"' as u8 {
        Rule::Char(chars_bytes[1] as char)
    } else {
        Rule::Subrules(chars.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect())
    }
}

fn rule_line(line: &str) -> (usize, Rule) {
    let mut colon = line.split(':');
    let index = colon.next().unwrap().parse::<usize>().unwrap();
    let mut options = colon.next().unwrap().split('|');
    let option1 = decode_option(options.next().unwrap());
    (index, if let Some(two) = options.next() {
        Rule::Alt(Box::from(option1), Box::from(decode_option(two)))
    } else {
        option1
    })
}

fn puzzle_2_rule_line(line: &str) -> (usize, Rule) {
    if line == "8: 42" {
        (8, Rule::Eight)
    } else if line == "11: 42 31" {
        (11, Rule::Eleven)
    } else {
        rule_line(line)
    }
}

impl Rules {
    pub fn from<F:Fn(&str)->(usize,Rule)>(filename: &str, rule_liner: F) -> io::Result<(Rules, impl Iterator<Item=String>)> {
        let mut lines = all_lines(filename)?;
        let rules = Rules {rules: lines.by_ref()
            .take_while(|line| line.len() > 0)
            .map(|line| rule_liner(line.as_str()))
            .collect()};
        Ok((rules, lines))
    }

    pub fn puzzle<F:Fn(&str)->(usize,Rule)>(filename: &str, rule_liner: F) -> io::Result<usize> {
        let (rules, lines) = Rules::from(filename, rule_liner)?;
        Ok(lines.filter(|line| rules.line_matcher(line.as_str())).count())
    }

    pub fn puzzle1(filename: &str) -> io::Result<usize> {
        Rules::puzzle(filename, rule_line)
    }

    pub fn puzzle2(filename: &str) -> io::Result<usize> {
        Rules::puzzle(filename, puzzle_2_rule_line)
    }

    pub fn line_matcher(&self, line: &str) -> bool {
        self.subline_matcher(self.rule(0), line.as_bytes(), 0)
            .map_or(false, |pos| pos == line.len())
    }

    pub fn subline_matcher(&self, r: &Rule, line: &[u8], pos: usize) -> Option<usize> {
        if pos < line.len() {
            match r {
                Rule::Char(c) => if *c == line[pos] as char { Some(pos + 1) } else { None },
                Rule::Alt(r1, r2) => self.subline_matcher(r1, line, pos).or(self.subline_matcher(r2, line, pos)),
                Rule::Subrules(subs) => self.process_subrules(subs, line, pos),
                Rule::Eight => self.eight_matcher(line, pos),
                Rule::Eleven => self.eleven_matcher(line, pos)
            }
        } else {
            None
        }
    }

    fn process_subrules(&self, subs: &SmallVec<[usize; 3]>, line: &[u8], pos: usize) -> Option<usize> {
        let mut result = self.subline_matcher(self.rule(subs[0]), line, pos);
        for i in 1..subs.len() {
            result = result.and_then(|pos| self.subline_matcher(self.rule(subs[i]), line, pos));
        }
        result
    }

    pub fn rule(&self, ri: usize) -> &Rule {
        self.rules.get(&ri).unwrap()
    }

    pub fn eight_matcher(&self, line: &[u8], pos: usize) -> Option<usize> {
        let mut result = self.subline_matcher(self.rule(42), line, pos);
        if let Some(next) = result {
            let mut new_pos = next;
            loop {
                let future = self.subline_matcher(self.rule(42), line, new_pos);
                if future == None {
                    line_printer("8 passed", line, pos);
                    return result;
                } else {
                    result = future;
                    new_pos = future.unwrap();
                }
            }
        } else {
            line_printer("8 failed", line, pos);
            None
        }
    }

    pub fn eleven_matcher(&self, line: &[u8], pos: usize) -> Option<usize> {
        if let Some(pos42) = self.subline_matcher(self.rule(42), line, pos) {
            let mut pos11 = pos42;
            loop {
                if let Some(new_pos) = self.eleven_matcher(line, pos11) {
                    pos11 = new_pos;
                } else {
                    break;
                }
            }
            if let Some(pos31) = self.subline_matcher(self.rule(31), line, pos11) {
                line_printer("11 passed", line, pos);
                return Some(pos31)
            }
        }
        line_printer("11 failed", line, pos);
        None
    }
}

fn line_printer(msg: &str, line: &[u8], pos: usize) {
    println!("{} on {:?}", msg, &line[pos..].iter().map(|b| *b as char).collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_1() {
        assert_eq!(Rules::puzzle1("in/day19_ex.txt").unwrap(), 2);
    }

    #[test]
    fn test1_2() {
        assert_eq!(Rules::puzzle1("in/day19_ex2.txt").unwrap(), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Rules::puzzle2("in/day19_ex2.txt").unwrap(), 12);
    }

    #[test]
    fn debug_11_1() {
        let rules = Rules::from("in/day19_ex2.txt", rule_line).unwrap().0;
        assert_ne!(rules.subline_matcher(rules.rule(0), "bbabbbbaabaabba".as_bytes(), 0), None);
        assert_ne!(rules.subline_matcher(rules.rule(11), "bbabbbbaabaabba".as_bytes(), 5), None);
    }

    #[test]
    fn debug_11_2() {
        let rules = Rules::from("in/day19_ex2.txt", puzzle_2_rule_line).unwrap().0;
        let after_8 = rules.eight_matcher("bbabbbbaabaabba".as_bytes(), 0);
        println!("after_8: {:?}", after_8);
        assert_ne!(after_8, None);
        assert_ne!(rules.subline_matcher(rules.rule(11), "bbabbbbaabaabba".as_bytes(), after_8.unwrap()), None);
    }
}