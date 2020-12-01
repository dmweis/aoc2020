use anyhow::Result;
use std::collections::HashMap;
use std::include_str;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error("FailedToFindPair")]
    FailedToFindPair,
}

fn input() -> &'static str {
    include_str!("inputs/day_1_task_1.txt")
}

fn task_1(input: &str) -> Result<i32> {
    let mut lookup = HashMap::new();

    let numbers = input
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok);

    for number in numbers {
        if let Some(other) = lookup.get(&number) {
            return Ok(other * number);
        }
        let key = 2020 - number;
        lookup.insert(key, number);
    }
    Err(Day1Error::FailedToFindPair.into())
}

fn task_2(input: &str) -> Result<i32> {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    for (i, first_number) in numbers.iter().enumerate() {
        for (o, second_number) in numbers[i..].iter().enumerate() {
            let position = i + o;
            for third_number in &numbers[position..] {
                let sum = first_number + second_number + third_number;
                if sum == 2020 {
                    return Ok(first_number * second_number * third_number);
                }
            }
        }
    }
    Err(Day1Error::FailedToFindPair.into())
}

pub fn run() {
    println!("Day 1 task 1 -> {}", task_1(input()).unwrap());
    println!("Day 1 task 2 -> {}", task_2(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = "1721
979
366
299
675
1456";
        let res = task_1(example).unwrap();
        assert_eq!(res, 514579);
    }

    #[test]
    fn task_1_input_1() {
        let input = input();
        let res = task_1(input).unwrap();
        assert_eq!(res, 864864);
    }

    #[test]
    fn example_2() {
        let example = "1721
979
366
299
675
1456";
        let res = task_2(example).unwrap();
        assert_eq!(res, 241861950);
    }

    #[test]
    fn task_2_input_1() {
        let input = input();
        let res = task_2(input).unwrap();
        assert_eq!(res, 281473080);
    }
}
