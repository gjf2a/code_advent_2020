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
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;

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
            "3_1" => day3::solve_1("in/day3.txt")?,
            "3_2" => day3::solve_2("in/day3.txt")?,
            "4_1" => day4::solve_1("in/day4.txt")?,
            "4_2" => day4::solve_2("in/day4.txt")?,
            "5_1" => day5::solve_1()?,
            "5_2" => day5::solve_2()?,
            "6_1" => day6::solve_1("in/day6.txt")?,
            "6_2" => day6::solve_2("in/day6.txt")?,
            "7_1" => day7::solve_1("in/day7.txt")?,
            "7_2" => day7::solve_2("in/day7.txt")?,
            "8_1" => day8::solve_1("in/day8.txt")?,
            "8_2" => day8::solve_2("in/day8.txt")?,
            "9_1" => day9::solve_1()?,
            "9_2" => day9::solve_2()?,
            "10_1" => day10::solve_1("in/day10.txt")?,
            "10_2" => day10::solve_2("in/day10.txt")?,
            "11_1" => day11::solve_1("in/day11.txt")?,
            "11_2" => day11::solve_2("in/day11.txt")?,
            "12_1" => day12::solve_1("in/day12.txt")?,
            "12_2" => day12::solve_2("in/day12.txt")?,
            "13_1" => day13::solve_1("in/day13.txt")?,
            "13_2" => day13::solve_2()?,
            "14_1" => day14::solve_1("in/day14.txt")?,
            "14_2" => day14::solve_2("in/day14.txt")?,
            "15_1" => day15::solve_1(),
            "15_2" => day15::solve_2(),
            "16_1" => day16::solve_1()?,
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}
