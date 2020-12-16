use std::collections::BTreeMap;
use std::io;
use advent_code_lib::all_lines;

pub fn solve_1() -> io::Result<String> {
    Ok(Notes::from("in/day16.txt")?.nearby_ticket_scanning_error_rate().to_string())
}

#[derive(Debug,Clone,Eq,PartialEq)]
struct Notes {
    fields: BTreeMap<String,((usize,usize),(usize,usize))>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>
}

impl Notes {
    pub fn from(filename: &str) -> io::Result<Self> {
        let mut lines = all_lines(filename)?.map(|s| s.unwrap());
        let fields: BTreeMap<String,((usize,usize),(usize,usize))> = lines.by_ref()
            .take_while(|line| line.len() > 0)
            .map(|line| parse_field_line(line.as_str()))
            .collect();
        let my_ticket = parse_ticket_line(lines.by_ref()
            .skip_while(|line| line.len() == 0 || line == "your ticket:")
            .next().unwrap().as_str());
        let nearby_tickets = lines
            .skip_while(|line| line.len() == 0 || line == "nearby tickets:")
            .map(|line| parse_ticket_line(line.as_str()))
            .collect();
        Ok(Notes {fields, my_ticket, nearby_tickets})
    }

    pub fn matches_range_for(&self, field: &str, value: usize) -> bool {
        match self.fields.get(field) {
            Some(((min1, max1), (min2, max2))) =>
                *min1 <= value && value <= *max1 || *min2 <= value && value <= *max2,
            None => false
        }
    }

    pub fn invalid_values_for(&self, ticket: &Vec<usize>) -> Vec<usize> {
        ticket.iter()
            .map(|n| *n)
            .filter(|value| !self.fields.keys()
                .any(|key| self.matches_range_for(key.as_str(), *value)))
            .collect()
    }

    pub fn nearby_ticket_scanning_error_rate(&self) -> usize {
        self.nearby_tickets.iter()
            .map(|t| self.invalid_values_for(t).iter().sum::<usize>())
            .sum()
    }
}

fn parse_field_line(line: &str) -> (String,((usize,usize),(usize,usize))) {
    let mut parts_colon = line.split(':');
    let field_name = parts_colon.next().unwrap().to_string();
    let ns: Vec<_> = parts_colon.next().unwrap().split(&[' ', '-', 'o','r'][..])
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    (field_name, ((ns[0], ns[1]), (ns[2], ns[3])))
}

fn parse_ticket_line(line: &str) -> Vec<usize> {
    line.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let notes = Notes::from("in/day16_ex.txt").unwrap();
        assert_eq!(notes.nearby_ticket_scanning_error_rate(), 71);
    }

    #[test]
    fn test_matches() {
        let notes = Notes::from("in/day16_ex.txt").unwrap();
        [(1,true), (2,true), (3,true), (4,false), (5,true), (7,true), (8,false)].iter()
            .for_each(|(v,tf)| {
                assert_eq!(notes.matches_range_for("class", *v), *tf);
            });
    }

    #[test]
    fn test_invalid_values() {
        let notes = Notes::from("in/day16_ex.txt").unwrap();
        assert_eq!(notes.invalid_values_for(&vec![40,4,50]), vec![4]);
    }

}