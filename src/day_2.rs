use anyhow::Result;
use std::include_str;
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
    min: u32,
    max: u32,
    letter: char,
    password: String,
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
            min,
            max,
            letter,
            password: (*password).to_owned(),
        })
    }

    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|letter| letter == &self.letter)
            .count() as u32;
        count >= self.min && count <= self.max
    }
}

fn task_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| PasswordAndPolicy::parse(line))
        .filter_map(Result::ok)
        .filter(|password_policy| password_policy.is_valid())
        .count()
}

// fn task_2(input: &str) -> Result<i32> {
//     let numbers: Vec<_> = input
//         .lines()
//         .map(|line| line.parse::<i32>())
//         .filter_map(Result::ok)
//         .collect();

//     for (i, first_number) in numbers.iter().enumerate() {
//         for (o, second_number) in numbers[i..].iter().enumerate() {
//             let position = i + o;
//             for third_number in &numbers[position..] {
//                 let sum = first_number + second_number + third_number;
//                 if sum == 2020 {
//                     return Ok(first_number * second_number * third_number);
//                 }
//             }
//         }
//     }
//     Err(Day1Error::FailedToFindPair.into())
// }

pub fn run() {
    println!("Day 2 task 1 -> {}", task_1(input()));
    // println!("Day 1 task 2 -> {}", task_2(input()).unwrap());
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
}
