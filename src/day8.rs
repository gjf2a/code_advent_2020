use std::io;
use crate::day8::Instruction::{Nop, Acc, Jmp};
use advent_code_lib::all_lines;
use std::collections::BTreeSet;

pub fn solve_1(filename: &str) -> io::Result<String> {
    Ok(terminates(CPU::from_file(filename)).1.to_string())
}

pub fn terminates(mut program: CPU) -> (bool,isize) {
    let mut visited = BTreeSet::new();
    while !program.terminated() && !visited.contains(&program.pc()) {
        visited.insert(program.pc());
        program.advance();
    }
    (program.terminated(), program.acc())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    let original_program = CPU::from_file(filename);
    for i in 0..original_program.len() {
        let mut fixed_copy = original_program.clone();
        fixed_copy.fix_instr(i);
        let (is_fixed, acc_value) = terminates(fixed_copy);
        if is_fixed {
            return Ok(acc_value.to_string());
        }
    }
    Ok("program cannot be fixed".to_owned())
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
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

    pub fn swap_jmp_nop(&self) -> Self {
        match &self {
            Nop(arg) => Jmp(*arg),
            Jmp(arg) => Nop(*arg),
            Acc(_) => *self
        }
    }
}

#[derive(Clone,Debug)]
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
            Nop(_) => {self.pc += 1;},
            Acc(arg) => {self.accumulator += arg; self.pc += 1;},
            Jmp(arg) => {self.pc = (self.pc as isize + arg) as usize;}
        }
    }

    pub fn terminated(&self) -> bool {
        self.pc >= self.program.len()
    }

    pub fn fix_instr(&mut self, i: usize) {
        self.program[i] = self.program[i].swap_jmp_nop();
    }

    pub fn pc(&self) -> usize {self.pc}
    pub fn acc(&self) -> isize {self.accumulator}
    pub fn len(&self) -> usize {self.program.len()}
    pub fn instr_at(&self, i: usize) -> Instruction {self.program[i]}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve_1() {
        assert_eq!(solve_1("day_8_example.txt").unwrap(), "5");
    }

    #[test]
    pub fn test_solve_2() {
        assert_eq!(solve_2("day_8_example.txt").unwrap(), "8");
    }
}