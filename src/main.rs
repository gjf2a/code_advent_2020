mod day1;
mod day2;

use std::{env,fs,io};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{}", match args.get(1) {
        None => "Usage: code_advent_2000 puzzle_num [other_args]*".to_string(),
        Some(arg) => match arg.as_str() {
            "1_1" => day1::solve_1()?,
            "1_2" => day1::solve_2()?,
            "2_1" => day2::solve_1()?,
            "2_2" => day2::solve_2()?,
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}

pub fn for_each_line<F: FnMut(&str) -> io::Result<()>>(filename: &str, mut line_processor: F) -> io::Result<()> {
    for line in io::BufReader::new(fs::File::open(filename)?).lines() {
        line_processor(line?.as_str())?;
    }
    Ok(())
}

pub fn file2nums(filename: &str) -> io::Result<Vec<isize>> {
    let mut nums = Vec::new();
    for_each_line(filename, |line| Ok(nums.push(line.parse::<isize>().unwrap())))?;
    Ok(nums)
}

pub fn pass_counter<F: Fn(&str) -> bool>(filename: &str, passes_check: F) -> io::Result<String> {
    let mut total = 0;
    for_each_line(filename, |line| Ok({
        if passes_check(line) {
            total += 1;
        }
    }))?;
    Ok(format!("{}", total))
}