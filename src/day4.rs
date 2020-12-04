use std::io;
use crate::for_each_line;
use std::collections::BTreeMap;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(format!("{}", solve_1_count(filename)?))
}

fn solve_1_count(filename: &str) -> io::Result<usize> {
    Ok(fields_and_values_from(filename)?.iter()
        .filter(|m| has_all_puzzle_1(*m))
        .count())
}

fn has_all_puzzle_1(passport: &BTreeMap<String,String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .all(|field| passport.contains_key(*field))
}

fn fields_and_values_from(filename: &str) -> io::Result<Vec<BTreeMap<String,String>>> {
    let mut result = Vec::new();
    let mut current = BTreeMap::new();
    for_each_line(filename, |line| Ok({
        let line = line.trim();
        if line.len() == 0 {
            result.push(current.clone());
            current = BTreeMap::new();
        } else {
            for pair in line.split_whitespace() {
                let mut parts = pair.split(':');
                let key = parts.next().unwrap();
                let value = parts.next().unwrap();
                current.insert(key.to_string(), value.to_string());
            }
        }
    }))?;

    result.push(current);
    Ok(result)
}

fn stringify_map(m: &BTreeMap<&str,&str>) -> BTreeMap<String,String> {
    m.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

fn stringify_vec_map(lit: Vec<BTreeMap<&str,&str>>) -> Vec<BTreeMap<String,String>> {
    lit.iter().map(|m| stringify_map(m)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let example_target = stringify_vec_map(vec![
            btreemap!("ecl"=>"gry", "pid"=>"860033327", "eyr"=>"2020",
            "hcl"=>"#fffffd", "byr"=>"1937", "iyr"=>"2017", "cid"=>"147", "hgt"=>"183cm"),
            btreemap!("iyr"=>"2013", "ecl"=>"amb", "cid"=>"350", "eyr"=>"2023",
            "pid"=>"028048884", "hcl"=>"#cfa07d", "byr"=>"1929"),
            btreemap!("hcl"=>"#ae17e1", "iyr"=>"2013", "eyr"=>"2024", "ecl"=>"brn",
            "pid"=>"760753108", "byr"=>"1931", "hgt"=>"179cm"),
            btreemap!("hcl"=>"#cfa07d", "eyr"=>"2025", "pid"=>"166559648",
            "iyr"=>"2011", "ecl"=>"brn", "hgt"=>"59in")
        ]);

        assert_eq!(fields_and_values_from("day_4_example.txt").unwrap(), example_target);
    }

    #[test]
    fn test_1_example() {
        assert_eq!(solve_1_count("day_4_example.txt").unwrap(), 2);
    }
}