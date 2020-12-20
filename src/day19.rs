use std::collections::{BTreeMap, BTreeSet};
use smallvec::SmallVec;
use std::{io, fmt};
use advent_code_lib::all_lines;
use std::collections::btree_map::Keys;
use std::fmt::Display;
use smallvec::alloc::fmt::Formatter;

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
}

#[derive(Clone,Eq,PartialEq,Debug)]
enum Status {
    Yes(BTreeSet<usize>), No, Pending
}

fn or(s1: Status, s2: Status) -> Status {
    match (s1, s2) {
        (Status::Yes(set1), Status::Yes(set2)) =>
            Status::Yes(set1.union(&set2).copied().collect()),
        (Status::Yes(set), _) | (_, Status::Yes(set)) => Status::Yes(set.clone()),
        (Status::Pending, _) | (_, Status::Pending) => Status::Pending,
        _ => Status::No
    }
}

#[derive(Clone,Eq,PartialEq,Debug)]
struct ParseTable {
    status: Vec<BTreeMap<usize,Status>>
}

impl Display for ParseTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.status.iter() {
            writeln!(f, "{:?}", line).unwrap();
        }
        Ok(())
    }
}

impl ParseTable {
    fn from(rules: &Rules, line: &str) -> ParseTable {
        let mut table = ParseTable {status: (0..line.len())
            .map(|_| rules.all_rule_nums()
                .map(|n| (*n, Status::Pending))
                .collect())
            .collect()};
        let line_bytes = line.as_bytes();
        for i in (0..line_bytes.len()).rev() {
            table.resolve_all(rules, line_bytes[i], i);
        }
        table
    }

    fn matches_rule_at(&self, rule: usize, at: usize) -> bool {
        match self.status[at].get(&rule).unwrap() {
            Status::Yes(matches) => matches.contains(&self.status.len()),
            _ => false
        }
    }

    fn full_match(&self) -> bool {
        self.matches_rule_at(0, 0)
    }

    fn matches(rules: &Rules, line: &str) -> bool {
        ParseTable::from(rules, line).full_match()
    }

    fn resolve_all(&mut self, rules: &Rules, c: u8, i: usize) {
        let mut has_updates = true;
        while has_updates {
            let updates = self.all_status_updates(rules, i, c);
            has_updates = updates.len() > 0;
            self.apply_status_updates(i, updates);
        }
        self.convert_pending_to_no(i);
    }

    fn all_status_updates(&self, rules: &Rules, i: usize, c: u8) -> Vec<(usize, Status)> {
        self.status[i].iter()
            .filter(|(_, v)| v == &&Status::Pending)
            .map(|(r, _)| (*r, self.get_new_status(rules.rule(*r), c, i)))
            .collect()
    }

    fn apply_status_updates(&mut self, i: usize, updates: Vec<(usize, Status)>) {
        for (r, status) in updates {
            self.status[i].insert(r, status);
        }
    }

    fn convert_pending_to_no(&mut self, i: usize) {
        for (_, r_status) in self.status[i].iter_mut() {
            if *r_status == Status::Pending {
                *r_status = Status::No;
            }
        }
    }

    fn status(&self, i: usize, rule: usize) -> Status {
        self.status.get(i).map_or(Status::No, |v| v.get(&rule).unwrap_or(&Status::No).clone())
    }

    fn get_new_status(&self, rule: &Rule, c: u8, i: usize) -> Status {
        match rule {
            Rule::Char(rc) => if c as char == *rc {Status::Yes(btreeset! {1})} else {Status::No},
            Rule::Subrules(subs) => self.subrule_stage(subs, 0, i),
            Rule::Alt(r1, r2) =>
                or(self.get_new_status(&r1, c, i), self.get_new_status(&r2, c, i))
        }
    }

    fn subrule_stage(&self, subs: &SmallVec<[usize;3]>, subrule: usize, i: usize) -> Status {
        match self.status(i, subs[subrule]) {
            Status::Yes(offsets) => if subrule + 1 < subs.len() {
                self.try_successors(offsets, subs, subrule + 1, i)
            } else {
                Status::Yes(offsets)
            },
            x => x
        }
    }

    fn try_successors(&self, current_offsets: BTreeSet<usize>, subs: &SmallVec<[usize;3]>, successor_subrule: usize, i: usize) -> Status {
        current_offsets.iter()
            .map(|off| match self.subrule_stage(subs, successor_subrule, i + off) {
                Status::Yes(future_offsets) => Status::Yes(future_offsets.iter().map(|fut| fut + off).collect()),
                other => other
            })
            .fold_first(|acc, val| or(acc, val)).unwrap()
    }
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
    fn test_row_1() {
        let rules = Rules::from("in/day19_ex.txt", rule_line).unwrap().0;
        let table = ParseTable::from(&rules, "ababbb");
        assert!(table.full_match());
    }

    #[test]
    fn test_rows_2() {
        let (rules,_) = Rules::from("in/day19_ex2.txt", puzzle_2_rule_line).unwrap();
        [
            ("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", false),
            ("bbabbbbaabaabba", true),
            ("babbbbaabbbbbabbbbbbaabaaabaaa", true),
            ("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", true),
            ("bbbbbbbaaaabbbbaaabbabaaa", true),
            ("bbbababbbbaaaaaaaabbababaaababaabab", true),
            ("ababaaaaaabaaab", true),
            ("ababaaaaabbbaba", true),
            ("baabbaaaabbaaaababbaababb", true),
            ("abbbbabbbbaaaababbbbbbaaaababb", true),
            ("aaaaabbaabaaaaababaa", true),
            ("aaaabbaaaabbaaa", false),
            ("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", true),
            ("babaaabbbaaabaababbaabababaaab", false),
            ("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", true)
        ].iter().for_each(|(msg, goal)| {
            println!("Testing '{}'", msg);
            //assert_eq!(ParseTable::matches(&rules, msg), *goal);
            let table = ParseTable::from(&rules, msg);
            println!("Expected: {} actual: {}", goal, table.full_match());
            if table.full_match() != *goal {
                println!("Failed; table:\n{}", table);
            }
        });
    }

    #[test]
    fn test_or() {
        assert_eq!(or(Status::Yes(btreeset! {2}), Status::No), Status::Yes(btreeset! {2}));
    }
}