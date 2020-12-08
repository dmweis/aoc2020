use anyhow::Result;
use std::collections::HashSet;
use thiserror::Error;

fn input() -> &'static str {
    include_str!("inputs/day_8.txt")
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("ParsingError")]
    ParsingError,
    #[error("UnknownInstruction")]
    UnknownInstruction,
    #[error("MissingInstruction")]
    MissingInstruction,
}
enum Command {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

impl Command {
    fn parse(input: &str) -> Result<Command> {
        let mut split = input.split_whitespace();
        let instruction = split.next().ok_or(DayError::ParsingError)?;
        let value: i32 = split.next().ok_or(DayError::ParsingError)?.parse()?;
        match instruction {
            "acc" => Ok(Command::Acc(value)),
            "nop" => Ok(Command::Nop(value)),
            "jmp" => Ok(Command::Jmp(value)),
            _ => Err(DayError::UnknownInstruction.into()),
        }
    }
}

struct Computer {
    commands: Vec<Command>,
}

impl Computer {
    fn parse(input: &str) -> Result<Computer> {
        let commands = input
            .lines()
            .map(|line| Command::parse(line))
            .collect::<Result<_>>()?;
        Ok(Computer { commands })
    }

    fn run_blocking_no_repeat(&mut self) -> Result<i32> {
        let mut visited = HashSet::new();
        let mut accumulator = 0;
        let mut instruction_pointer = 0;
        loop {
            if !visited.insert(instruction_pointer) {
                break;
            }
            let instruction = self
                .commands
                .get(instruction_pointer)
                .ok_or(DayError::MissingInstruction)?;
            match instruction {
                Command::Nop(_) => instruction_pointer += 1,
                Command::Acc(val) => {
                    accumulator += val;
                    instruction_pointer += 1;
                }
                Command::Jmp(relative_jump) => {
                    instruction_pointer = (instruction_pointer as i32 + relative_jump) as usize
                }
            }
        }
        Ok(accumulator)
    }
}

fn task_1(input: &str) -> Result<i32> {
    Ok(Computer::parse(input)?.run_blocking_no_repeat()?)
}

pub fn run() {
    println!("Day 8 task 1 -> {}", task_1(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_example_1() {
        let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let mut computer = Computer::parse(example).unwrap();
        let result = computer.run_blocking_no_repeat().unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn task_1_test() {
        let res = task_1(input()).unwrap();
        assert_eq!(res, 1610);
    }
}
