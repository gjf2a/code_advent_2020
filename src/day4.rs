use std::io;
use crate::for_each_line;
use std::collections::BTreeMap;

pub fn solve_1(filename: &str) -> io::Result<String> {
    solve(filename, has_all_puzzle_1)
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    solve(filename, valid_puzzle_2)
}

fn solve<P: Fn(&BTreeMap<String,String>) -> bool>(filename: &str, predicate: P) -> io::Result<String> {
    Ok(format!("{}", solve_count(filename, predicate)?))
}

fn solve_count<P: Fn(&BTreeMap<String,String>) -> bool>(filename: &str, predicate: P) -> io::Result<usize> {
    Ok(fields_and_values_from(filename)?.iter()
        .filter(|m| predicate(*m))
        .count())
}

fn has_all_puzzle_1(passport: &BTreeMap<String,String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .all(|field| passport.contains_key(*field))
}

fn valid_puzzle_2(passport: &BTreeMap<String,String>) -> bool {
    passport.iter().all(|(k, v)| valid_field(k.as_str(), v.as_str()))
}

fn valid_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => validate_year(value, 1920, 2002),
        "iyr" => validate_year(value, 2010, 2020),
        "eyr" => validate_year(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair(value),
        "ecl" => btreeset!("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(value),
        "pid" => validate_pid(value),
        "cid" => true,
        _ => false
    }
}

fn validate_year(value: &str, min: usize, max: usize) -> bool {
    value.len() == 4 && in_range(value, min, max)
}

fn validate_height(value: &str) -> bool {
    if value.ends_with("cm") {
        in_range(&value[..value.len() - 2], 150, 193)
    } else if value.ends_with("in") {
        in_range(&value[..value.len() - 2], 59, 76)
    } else {
        false
    }
}

fn validate_hair(value: &str) -> bool {
    let mut v_iter = value.chars();
    match v_iter.next() {
        None => false,
        Some(c) => c == '#' && v_iter.all(|c| c.is_digit(16))
    }
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9 && value.chars().all(|d| d.is_digit(10))
}

fn in_range(value: &str, min: usize, max: usize) -> bool {
    match value.parse::<usize>() {
        Err(_) => false,
        Ok(value) => value >= min && value <= max
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify_map(m: &BTreeMap<&str,&str>) -> BTreeMap<String,String> {
        m.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    fn stringify_vec_map(lit: Vec<BTreeMap<&str,&str>>) -> Vec<BTreeMap<String,String>> {
        lit.iter().map(|m| stringify_map(m)).collect()
    }

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
        assert_eq!(solve_1("day_4_example.txt").unwrap(), "2");
    }

    #[test]
    fn test_2_values() {
        for (field, value) in [("byr", "2002"), ("hgt", "60in"), ("hgt", "190cm"),
            ("hcl", "#123abc"), ("ecl", "brn"), ("pid", "000000001")].iter() {
            assert!(valid_field(field, value));
        }

        for (field, value) in [("byr", "2003"), ("hgt", "190in"),
            ("hgt", "190"), ("hcl", "#123abz"), ("hcl", "123abc"), ("ecl", "wat"),
            ("pid", "0123456789")].iter() {
            assert!(!valid_field(field, value));
        }
    }

    #[test]
    fn test_2_example_2() {
        assert_eq!(solve_2("day_4_example_2.txt").unwrap(), "4");
    }
}