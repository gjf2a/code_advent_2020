#[macro_use] extern crate maplit;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::{env,fs,io};
use std::io::{BufRead, Lines, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{}", match args.get(1) {
        None => "Usage: code_advent_2000 puzzle_num [other_args]*".to_string(),
        Some(arg) => match arg.as_str() {
            "1_1" => day1::solve_1()?,
            "1_2" => day1::solve_2()?,
            "2_1" => day2::solve_1()?,
            "2_2" => day2::solve_2()?,
            "3_1" => day3::solve_1("day_3_input.txt")?,
            "3_2" => day3::solve_2("day_3_input.txt")?,
            "4_1" => day4::solve_1("day_4_input.txt")?,
            "4_2" => day4::solve_2("day_4_input.txt")?,
            "5_1" => day5::solve_1()?,
            "5_2" => day5::solve_2()?,
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}

pub fn all_lines(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    Ok(io::BufReader::new(fs::File::open(filename)?).lines())
}

pub fn for_each_line<F: FnMut(&str) -> io::Result<()>>(filename: &str, mut line_processor: F) -> io::Result<()> {
    for line in all_lines(filename)? {
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