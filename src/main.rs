#[macro_use] extern crate maplit;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::{env,fs,io};
use std::io::{BufRead, Lines, BufReader};
use std::fs::File;
use std::slice::Iter;

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
            "6_1" => day6::solve_1("day_6_input.txt")?,
            "6_2" => day6::solve_2("day_6_input.txt")?,
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
    Ok(all_lines(filename)?.map(|line| line.unwrap().parse::<isize>().unwrap()).collect())
}

pub fn pass_counter<F: Fn(&str) -> bool>(filename: &str, passes_check: F) -> io::Result<String> {
    Ok(all_lines(filename)?
        .filter(|line| line.as_ref().map_or(false, |line| passes_check(line.as_str())))
        .count().to_string())
}

pub struct MultiLineObjects<T: Eq+PartialEq+Clone> {
    objects: Vec<T>,
    maker: Box<dyn Fn()->T>
}

impl <T: Eq+PartialEq+Clone> MultiLineObjects<T> {
    pub fn new(starter: Box<dyn Fn()->T>) -> Self {
        MultiLineObjects {objects: vec![starter()], maker: starter}
    }

    pub fn from_file<P: FnMut(&mut T,&str)>(starter: Box<dyn Fn()->T>, filename: &str, proc: &mut P) -> io::Result<Self> {
        let mut result = MultiLineObjects::new(starter);
        for_each_line(filename, |line| Ok({
            result.add_line(line, proc);
        }))?;
        Ok(result)
    }

    pub fn add_line<P: FnMut(&mut T,&str)>(&mut self, line: &str, proc: &mut P) {
        let line = line.trim();
        if line.len() == 0 {
            self.objects.push((*self.maker)());
        } else {
            let end = self.objects.len() - 1;
            proc(&mut self.objects[end], line);
        }
    }

    pub fn objects(&self) -> Vec<T> {self.objects.clone()}

    pub fn iter(&self) -> Iter<T> {
        self.objects.iter()
    }

    pub fn count_matching<P: Fn(&T) -> bool>(&self, predicate: P) -> usize {
        self.iter()
            .filter(|m| predicate(*m))
            .count()
    }
}