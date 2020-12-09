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
    #[error("FailedToMutate")]
    FailedToMutate,
    #[error("InfiniteLoop")]
    InfiniteLoop,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

    fn run_blocking_correct_termination(&mut self) -> Result<i32> {
        let mut visited = HashSet::new();
        let mut accumulator = 0;
        let mut instruction_pointer = 0;
        loop {
            if !visited.insert(instruction_pointer) {
                return Err(DayError::InfiniteLoop.into());
            }
            if instruction_pointer == self.commands.len() {
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

struct ComputerMutator {
    original: Computer,
    next_mutate: usize,
}

impl ComputerMutator {
    fn new(computer: Computer) -> Self {
        ComputerMutator {
            original: computer,
            next_mutate: 0,
        }
    }

    fn verify_next(&mut self) -> Result<Option<i32>> {
        let max_instructions = self.original.commands.len();
        let next_mutate = self.next_mutate;
        self.next_mutate += 1;
        if next_mutate >= max_instructions {
            return Err(DayError::FailedToMutate.into());
        }

        let mut computer_clone = self.original.clone();
        let mutable_instruction = computer_clone
            .commands
            .get_mut(next_mutate)
            .ok_or(DayError::MissingInstruction)?;
        match &mutable_instruction {
            Command::Jmp(val) => *mutable_instruction = Command::Nop(*val),
            Command::Nop(val) => *mutable_instruction = Command::Jmp(*val),
            _ => return Ok(None),
        }
        Ok(computer_clone.run_blocking_correct_termination().ok())
    }
}

fn task_1(input: &str) -> Result<i32> {
    Ok(Computer::parse(input)?.run_blocking_no_repeat()?)
}

fn task_2(input: &str) -> Result<i32> {
    let computer = Computer::parse(input)?;
    let mut mutator = ComputerMutator::new(computer);
    loop {
        if let Some(res) = mutator.verify_next()? {
            return Ok(res);
        }
    }
}

pub fn run() {
    println!("Day 8 task 1 -> {}", task_1(input()).unwrap());
    println!("Day 8 task 2 -> {}", task_2(input()).unwrap());
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
    fn task_2_corrected_example() {
        let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
        let mut computer = Computer::parse(example).unwrap();
        let result = computer.run_blocking_correct_termination().unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn task_2_example_1() {
        let example = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let result = task_2(example).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn task_1_test() {
        let res = task_1(input()).unwrap();
        assert_eq!(res, 1610);
    }

    #[test]
    fn task_2_test() {
        let res = task_2(input()).unwrap();
        assert_eq!(res, 1703);
    }
}
