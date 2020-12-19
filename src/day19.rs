use std::collections::{BTreeMap, BTreeSet};
use smallvec::SmallVec;
use std::io;
use advent_code_lib::all_lines;
use std::collections::btree_map::Keys;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(Rules::puzzle1(filename)?.to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(Rules::puzzle2(filename)?.to_string())
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

fn puzzle_2_rule_line(line: &str) -> (usize, Rule) {
    rule_line(if line == "8: 42" {
        "8: 42 | 42 8"
    } else if line == "11: 42 31" {
        "11: 42 31 | 42 11 31"
    } else {
        line
    })
}

impl Rules {
    fn from<F:Fn(&str)->(usize,Rule)>(filename: &str, rule_liner: F) -> io::Result<(Rules, impl Iterator<Item=String>)> {
        let mut lines = all_lines(filename)?;
        let rules = Rules {rules: lines.by_ref()
            .take_while(|line| line.len() > 0)
            .map(|line| rule_liner(line.as_str()))
            .collect()};
        Ok((rules, lines))
    }

    fn all_rule_nums(&self) -> Keys<usize,Rule> {
        self.rules.keys()
    }

    /*
    fn puzzle<F:Fn(&str)->(usize,Rule)>(filename: &str, rule_liner: F) -> io::Result<usize> {
        let (rules, lines) = Rules::from(filename, rule_liner)?;
        Ok(lines.filter(|line| rules.line_matcher(line.as_str())).count())
    }
     */

    fn puzzle<F:Fn(&str)->(usize,Rule)>(filename: &str, rule_liner: F) -> io::Result<usize> {
        let (rules, lines) = Rules::from(filename, rule_liner)?;
        Ok(lines.filter(|line| ParseTable::matches(&rules, line.as_str())).count())
    }

    fn puzzle1(filename: &str) -> io::Result<usize> {
        Rules::puzzle(filename, rule_line)
    }

    fn puzzle2(filename: &str) -> io::Result<usize> {
        Rules::puzzle(filename, puzzle_2_rule_line)
    }

    fn rule(&self, ri: usize) -> &Rule {
        self.rules.get(&ri).unwrap()
    }
/*
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
*/
}

#[derive(Clone,Eq,PartialEq,Debug)]
enum Status {
    Yes(BTreeSet<usize>), No, Pending
}

fn and(s1: Status, s2: Status) -> Status {
    match (s1, s2) {
        (Status::Yes(set1), Status::Yes(set2)) => Status::Yes({
            let mut result = BTreeSet::new();
            for len1 in set1.iter() {
                for len2 in set2.iter() {
                    result.insert(len1 + len2);
                }
            }
            result
        }),
        (Status::No, _) | (_, Status::No) => Status::No,
        _ => Status::Pending,
    }
}

fn or(s1: Status, s2: Status) -> Status {
    match (s1, s2) {
        (Status::Yes(set1), Status::Yes(set2)) => Status::Yes(set1.union(&set2).map(|x| *x).collect()),
        (Status::Yes(set), Status::Pending)
        | (Status::Pending, Status::Yes(set)) => Status::Yes(set.clone()),
        (Status::Pending, _) | (_, Status::Pending) => Status::Pending,
        _ => Status::No
    }
}

#[derive(Clone,Eq,PartialEq,Debug)]
struct ParseTable {
    status: Vec<BTreeMap<usize,Status>>
}

impl ParseTable {
    fn from(rules: &Rules, line: &str) -> ParseTable {
        let mut table = ParseTable {status: (0..line.len()).map(|_| rules.all_rule_nums().map(|n| (*n, Status::Pending)).collect()).collect()};
        let line_bytes = line.as_bytes();
        for i in (0..line_bytes.len()).rev() {
            table.resolve_all(rules, line_bytes[i], i);
        }
        table
    }

    fn match_at_with(&self, at: usize, with: usize) -> bool {
        self.status[at].iter()
            .any(|(_, v)| match v {Status::Yes(n) => n.contains(&with), _ => false})
    }

    fn matches(rules: &Rules, line: &str) -> bool {
        let table = ParseTable::from(rules, line);
        println!("{:?}", table);
        table.match_at_with(0, line.len())
    }

    fn resolve_all(&mut self, rules: &Rules, c: u8, i: usize) {
        loop {
            let updates: Vec<(usize, Status)> = self.status[i].iter()
                .filter(|(_, v)| v == &&Status::Pending)
                .map(|(r, _)| (*r, self.get_new_status(rules.rule(*r), c, i)))
                .collect();
            let changed = updates.len() > 0;
            for (r, status) in updates {
                self.status[i].insert(r, status);
            }
            if !changed {
                for (_, r_status) in self.status[i].iter_mut() {
                    if *r_status == Status::Pending {
                        *r_status = Status::No;
                    }
                }
                return;
            }
        }
    }

    fn status(&self, i: usize, rule: usize) -> Status {
        self.status.get(i).map_or(Status::No, |v| v.get(&rule).unwrap_or(&Status::No).clone())
    }

    fn get_new_status(&self, rule: &Rule, c: u8, i: usize) -> Status {
        match rule {
            Rule::Char(rc) => if c as char == *rc {Status::Yes(btreeset! {1})} else {Status::No},
            Rule::Subrules(subs) => subs.iter().enumerate()
                .map(|(offset, subrule)| self.status(i+offset, *subrule))
                .fold_first(|acc, val| and(acc, val)).unwrap(),
            Rule::Alt(r1, r2) =>
                or(self.get_new_status(&r1, c, i), self.get_new_status(&r2, c, i))
        }
    }
}
/*
fn line_printer(msg: &str, line: &[u8], pos: usize) {
    println!("{} on {:?}", msg, &line[pos..].iter().map(|b| *b as char).collect::<String>());
}
*/
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

    /*
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

     */
}