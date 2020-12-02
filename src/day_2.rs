use anyhow::Result;
use std::include_str;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day2Error {
    #[error("FailedToParsePolicy")]
    FailedToParsePolicy,
}

fn input() -> &'static str {
    include_str!("inputs/day_2.txt")
}

struct PasswordAndPolicy {
    first_number: u32,
    second_number: u32,
    letter: char,
    password: String,
}

impl FromStr for PasswordAndPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PasswordAndPolicy::parse(s)
    }
}

impl PasswordAndPolicy {
    fn parse(input: &str) -> Result<Self> {
        let segments: Vec<&str> = input.split(' ').collect();
        let range: Vec<&str> = segments
            .get(0)
            .ok_or(Day2Error::FailedToParsePolicy)?
            .split('-')
            .collect();
        let min = range
            .get(0)
            .ok_or(Day2Error::FailedToParsePolicy)?
            .parse::<u32>()?;
        let max = range
            .get(1)
            .ok_or(Day2Error::FailedToParsePolicy)?
            .parse::<u32>()?;
        let letter = segments
            .get(1)
            .ok_or(Day2Error::FailedToParsePolicy)?
            .replace(':', "")
            .chars()
            .next()
            .ok_or(Day2Error::FailedToParsePolicy)?;
        let password = segments.get(2).ok_or(Day2Error::FailedToParsePolicy)?;
        Ok(PasswordAndPolicy {
            first_number: min,
            second_number: max,
            letter,
            password: (*password).to_owned(),
        })
    }

    fn is_valid_first_policy(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|letter| letter == &self.letter)
            .count() as u32;
        count >= self.first_number && count <= self.second_number
    }

    fn is_valid_second_policy(&self) -> bool {
        let first_index = self.first_number - 1;
        let second_index = self.second_number - 1;
        let first_correct = self
            .password
            .chars()
            .nth(first_index as usize)
            .filter(|letter| letter == &self.letter)
            .is_some();
        let second_correct = self
            .password
            .chars()
            .nth(second_index as usize)
            .filter(|letter| letter == &self.letter)
            .is_some();
        first_correct && !second_correct || !first_correct && second_correct
    }
}

fn task_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<PasswordAndPolicy>())
        .filter_map(Result::ok)
        .filter(|password_policy| password_policy.is_valid_first_policy())
        .count()
}

fn task_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<PasswordAndPolicy>())
        .filter_map(Result::ok)
        .filter(|password_policy| password_policy.is_valid_second_policy())
        .count()
}

pub fn run() {
    println!("Day 2 task 1 -> {}", task_1(input()));
    println!("Day 2 task 2 -> {}", task_2(input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let res = task_1(example);
        assert_eq!(res, 2);
    }

    #[test]
    fn task_1_test() {
        let example = input();
        let res = task_1(example);
        assert_eq!(res, 458);
    }

    #[test]
    fn example_2() {
        let example = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let res = task_2(example);
        assert_eq!(res, 1);
    }

    #[test]
    fn validate_policy_2_correct() {
        let res = PasswordAndPolicy::parse("1-3 a: abcde").unwrap();
        assert!(res.is_valid_second_policy());
    }

    #[test]
    fn validate_policy_2_none() {
        let res = PasswordAndPolicy::parse("1-3 b: cdefg").unwrap();
        assert!(!res.is_valid_second_policy());
    }

    #[test]
    fn validate_policy_2_both() {
        let res = PasswordAndPolicy::parse("2-9 c: ccccccccc").unwrap();
        assert!(!res.is_valid_second_policy());
    }

    #[test]
    fn task_2_test() {
        let example = input();
        let res = task_2(example);
        assert_eq!(res, 342);
    }
}
