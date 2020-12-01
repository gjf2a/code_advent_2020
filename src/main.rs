use std::{env,fs,io};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{}", match args.get(1) {
        None => "Usage: code_advent_2000 puzzle_num [other_args]*".to_string(),
        Some(arg) => match arg.as_str() {
            "1" => solve_day_1()?,
            "1a" => solve_day_1a()?,
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}

fn file2nums(filename: &str) -> io::Result<Vec<isize>> {
    let reader = io::BufReader::new(fs::File::open(filename)?);
    Ok(reader.lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect())
}

fn solve_day_1() -> io::Result<String> {
    let nums = file2nums("day_1_input.txt")?;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                return Ok(format!("{} + {} == 2020; {} * {} == {}", nums[i], nums[j], nums[i], nums[j], nums[i] * nums[j]));
            }
        }
    }
    Ok("Failed".to_string())
}

fn solve_day_1a() -> io::Result<String> {
    let nums = file2nums("day_1_input.txt")?;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return Ok(format!("{} + {} + {} == 2020; {} * {} * {} == {}", nums[i], nums[j], nums[k], nums[i], nums[j], nums[k], nums[i] * nums[j] * nums[k]));
                }
            }
        }
    }
    Ok("Failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_day_1().unwrap().as_str(), "1477 + 543 == 2020; 1477 * 543 == 802011")
    }

    #[test]
    fn test_1a() {
        assert_eq!(solve_day_1a().unwrap().as_str(), "422 + 577 + 1021 == 2020; 422 * 577 * 1021 == 248607374")
    }

}