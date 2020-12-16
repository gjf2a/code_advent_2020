use std::collections::{BTreeMap, BTreeSet};
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1() -> io::Result<String> {
    Ok(Notes::from("in/day16.txt", true)?.nearby_ticket_scanning_error_rate().to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(Notes::from(filename, false)?.departure_product().to_string())
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Constraints {
    fields: BTreeMap<String,((usize,usize),(usize,usize))>
}

impl Constraints {
    pub fn from(lines: &mut impl Iterator<Item=String>) -> Self {
        Constraints {fields: lines.take_while(|line| line.len() > 0)
            .map(|line| parse_field_line(line.as_str()))
            .collect()}
    }

    pub fn names(&self) -> impl Iterator<Item=&String> {
        self.fields.keys()
    }

    pub fn matches_range_for(&self, field: &str, value: usize) -> bool {
        let ((min1, max1), (min2, max2)) = self.fields.get(field).unwrap();
        *min1 <= value && value <= *max1 || *min2 <= value && value <= *max2
    }

    pub fn some_field_accepts(&self, value: usize) -> bool {
        self.names().any(|key| self.matches_range_for(key.as_str(), value))
    }

    pub fn accepts_ticket(&self, ticket: &Vec<usize>) ->  bool {
        ticket.iter().all(|v| self.some_field_accepts(*v))
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Notes {
    fields: Constraints,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>
}

impl Notes {
    fn from(filename: &str, allow_invalid: bool) -> io::Result<Self> {
        let mut lines = all_lines(filename)?;
        let fields = Constraints::from(&mut lines.by_ref());
        let my_ticket = parse_ticket_line(lines.by_ref()
            .skip_while(|line| line.len() == 0 || line == "your ticket:")
            .next().unwrap().as_str());
        let nearby_tickets = lines
            .skip_while(|line| line.len() == 0 || line == "nearby tickets:")
            .map(|line| parse_ticket_line(line.as_str()))
            .filter(|t| allow_invalid || fields.accepts_ticket(t))
            .collect();
        Ok(Notes {fields, my_ticket, nearby_tickets})
    }

    pub fn matches_range_for(&self, field: &str, value: usize) -> bool {
        self.fields.matches_range_for(field, value)
    }

    pub fn invalid_values_for(&self, ticket: &Vec<usize>) -> Vec<usize> {
        ticket.iter()
            .map(|n| *n)
            .filter(|value| !self.fields.some_field_accepts(*value))
            .collect()
    }

    pub fn potential_positions(&self) -> BTreeMap<String,BTreeSet<usize>> {
        self.fields.names().map(|k| (k.clone(), (0..self.num_positions()).collect())).collect()
    }

    pub fn my_field_values(&self) -> BTreeMap<String,usize> {
        self.field_positions().iter().map(|(k,v)| (k.clone(), self.my_ticket[*v])).collect()
    }

    pub fn my_departures(&self) -> BTreeMap<String,usize> {
        self.my_field_values().iter()
            .filter(|(k,_)| k.starts_with("departure"))
            .map(|(k,v)| (k.clone(), *v))
            .collect()
    }

    pub fn num_positions(&self) -> usize {
        self.my_ticket.len()
    }

    pub fn field_positions(&self) -> BTreeMap<String,usize> {
        PotentialMatches::get_matches_from(self)
    }

    pub fn nearby_ticket_scanning_error_rate(&self) -> usize {
        self.nearby_tickets.iter()
            .map(|t| self.invalid_values_for(t).iter().sum::<usize>())
            .sum()
    }

    pub fn departure_product(&self) -> usize {
        self.my_departures().values().product()
    }
}

#[derive(Debug,Clone)]
struct PotentialMatches {
    candidates: BTreeMap<String,BTreeSet<usize>>,
    matches: BTreeMap<String,usize>
}

impl PotentialMatches {
    fn get_matches_from(notes: &Notes) -> BTreeMap<String,usize> {
        let mut potential = PotentialMatches { candidates: notes.potential_positions(), matches: BTreeMap::new()};
        potential.remove_impossible(notes);
        while potential.candidates.len() > 0 {
            potential.assign_most_constrained()
        }
        potential.matches
    }

    fn remove_impossible(&mut self, notes: &Notes) {
        for ticket in notes.nearby_tickets.iter() {
            for (field, positions) in self.candidates.iter_mut() {
                for p in 0..ticket.len() {
                    if !notes.matches_range_for(field.as_str(), ticket[p]) {
                        positions.remove(&p);
                    }
                }
            }
        }
    }

    fn assign_most_constrained(&mut self) {
        let (_, next, position) = self.candidates.iter()
            .map(|(k,v)| (v.len(), k.clone(), *v.iter().next().unwrap()))
            .min().unwrap();
        self.candidates.remove(next.as_str());
        self.matches.insert(next, position);
        for candidate in self.candidates.values_mut() {
            candidate.remove(&position);
        }
    }
}

fn parse_field_line(line: &str) -> (String,((usize,usize),(usize,usize))) {
    let mut parts_colon = line.split(':');
    let field_name = parts_colon.next().unwrap().to_string();
    let ns: Vec<_> = parts_colon.next().unwrap().split(&[' ', '-', 'o','r'][..])
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().unwrap())
        .collect();
    (field_name, ((ns[0], ns[1]), (ns[2], ns[3])))
}

fn parse_ticket_line(line: &str) -> Vec<usize> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let notes = Notes::from("in/day16_ex1.txt", true).unwrap();
        assert_eq!(notes.nearby_ticket_scanning_error_rate(), 71);
    }

    #[test]
    fn test_matches() {
        let notes = Notes::from("in/day16_ex1.txt", true).unwrap();
        [(1,true), (2,true), (3,true), (4,false), (5,true), (7,true), (8,false)].iter()
            .for_each(|(v,tf)| {
                assert_eq!(notes.matches_range_for("class", *v), *tf);
            });
    }

    #[test]
    fn test_invalid_values() {
        let notes = Notes::from("in/day16_ex1.txt", true).unwrap();
        assert_eq!(notes.invalid_values_for(&vec![40,4,50]), vec![4]);
    }

    #[test]
    fn test_field_positions() {
        let notes = Notes::from("in/day16_ex2.txt", true).unwrap();
        assert_eq!(notes.field_positions(), btreemap! {"class".to_string() => 1, "row".to_string() => 0, "seat".to_string() => 2});
    }

    #[test]
    fn test_my_fields() {
        let notes = Notes::from("in/day16_ex2.txt", true).unwrap();
        assert_eq!(notes.my_field_values(), btreemap! {"class".to_string() => 12, "row".to_string() => 11, "seat".to_string() => 13});
    }

    #[test]
    fn test_valid_field_positions() {
        let notes = Notes::from("in/day16.txt", false).unwrap();
        let unique_positions: BTreeSet<usize> = notes.field_positions().iter().map(|p| *p.1).collect();
        assert_eq!(unique_positions.len(), notes.num_positions());
    }

    #[test]
    fn test_departures() {
        let notes = Notes::from("in/day16.txt", false).unwrap();
        assert_eq!(format!("{:?}", notes.my_departures()), r#"{"departure date": 101, "departure location": 53, "departure platform": 89, "departure station": 61, "departure time": 113, "departure track": 73}"#)
    }
}