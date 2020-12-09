#[macro_use] extern crate maplit;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::{env,io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{}", match args.get(1) {
        None => "Usage: code_advent_2020 puzzle_num [other_args]*".to_string(),
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
            "6_1" => day6::solve_1("day_6_input.txt")?,
            "6_2" => day6::solve_2("day_6_input.txt")?,
            "7_1" => day7::solve_1("day_7_input.txt")?,
            "7_2" => day7::solve_2("day_7_input.txt")?,
            "8_1" => day8::solve_1("day_8_input.txt")?,
            "8_2" => day8::solve_2("day_8_input.txt")?,
            "9_1" => day9::solve_1()?,
            "9_2" => day9::solve_2()?,
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}
