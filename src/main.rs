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
            _ => "Unrecognized problem".to_string()
        }
    });
    Ok(())
}

pub fn file2nums(filename: &str) -> io::Result<Vec<isize>> {
    let reader = io::BufReader::new(fs::File::open(filename)?);
    Ok(reader.lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect())
}