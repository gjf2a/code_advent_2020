use std::collections::BTreeMap;
use smallvec::SmallVec;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Rules::puzzle1(filename)?.to_string())
}

#[derive(Clone,Debug)]
enum Rule {
    Char(char), Subrules(SmallVec<[usize; 3]>), Alt(Box<Rule>,Box<Rule>)
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

impl Rules {
    pub fn puzzle1(filename: &str) -> io::Result<usize> {
        let mut lines = all_lines(filename)?;
        let rules = Rules {rules: lines.by_ref()
            .take_while(|line| line.len() > 0)
            .map(|line| rule_line(line.as_str()))
            .collect()};
        Ok(lines.filter(|line| rules.puzzle1_match(line.as_str())).count())
    }

    pub fn puzzle1_match(&self, line: &str) -> bool {
        self.matches(self.rule(0), line.as_bytes(), 0)
            .map_or(false, |pos| pos == line.len())
    }

    pub fn matches(&self, r: &Rule, line: &[u8], pos: usize) -> Option<usize> {
        match r {
            Rule::Char(c) => if *c == line[pos] as char {Some(pos + 1)} else {None},
            Rule::Alt(r1, r2) => self.matches(r1, line, pos).or(self.matches(r2, line, pos)),
            Rule::Subrules(subs) => self.process_subrules(subs, line, pos)
        }
    }

    fn process_subrules(&self, subs: &SmallVec<[usize; 3]>, line: &[u8], pos: usize) -> Option<usize> {
        let mut result = self.matches(self.rule(subs[0]), line, pos);
        for i in 1..subs.len() {
            result = result.and_then(|pos| self.matches(self.rule(subs[i]), line, pos));
        }
        result
    }

    pub fn rule(&self, ri: usize) -> &Rule {
        self.rules.get(&ri).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Rules::puzzle1("in/day19_ex.txt").unwrap(), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Rules::puzzle1("in/day19_ex2.txt").unwrap(), 3);
    }

}