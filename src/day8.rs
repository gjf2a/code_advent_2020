use std::io;
use crate::day8::Instruction::{Nop, Acc, Jmp};
use advent_code_lib::all_lines;
use std::collections::BTreeSet;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let mut program = CPU::from_file(filename);
    let mut visited = BTreeSet::new();
    while !visited.contains(&program.pc()) {
        visited.insert(program.pc());
        program.advance();
    }
    Ok(program.acc().to_string())
}

pub enum Instruction {
    Nop(isize), Acc(isize), Jmp(isize)
}

impl Instruction {
    pub fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split_whitespace().collect();
        let arg = parts[1].parse::<isize>().unwrap();
        match parts[0] {
            "nop" => Nop(arg),
            "acc" => Acc(arg),
            "jmp" => Jmp(arg),
            _ => panic!("Did not recognize `{}`", parts[0])
        }
    }
}

pub struct CPU {
    program: Vec<Instruction>,
    pc: usize,
    accumulator: isize
}

impl CPU {
    pub fn from_file(filename: &str) -> Self {
        CPU {program: all_lines(filename).unwrap()
            .map(|line| Instruction::from(line.unwrap().as_str()))
            .collect(),
            pc: 0,
            accumulator: 0}
    }

    pub fn advance(&mut self) {
        match self.program[self.pc] {
            Nop(_) => {},
            Acc(arg) => {self.accumulator += arg},
            Jmp(arg) => {self.pc = (self.pc as isize + arg - 1) as usize;}
        }
        self.pc += 1;
    }

    pub fn pc(&self) -> usize {self.pc}
    pub fn acc(&self) -> isize {self.accumulator}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve_1() {
        assert_eq!(solve_1("day_8_example.txt").unwrap(), "5");
    }

}