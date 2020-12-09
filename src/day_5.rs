use std::collections::HashSet;

use anyhow::Result;
use thiserror::Error;

const FRONT: char = 'F';
const BACK: char = 'B';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn input() -> &'static str {
    include_str!("inputs/day_5.txt")
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("FailedParsingPosition")]
    FailedParsingPosition,
    #[error("Failed to find seat")]
    FailedToFindSeat,
}

fn calculate_row_rec(input: &str, min: usize, max: usize) -> Result<usize> {
    match input.chars().next() {
        Some(FRONT) => calculate_row_rec(&input[1..], min, min + (max - min) / 2),
        Some(BACK) => calculate_row_rec(&input[1..], min + 1 + (max - min) / 2, max),
        Some(_) => Err(DayError::FailedParsingPosition.into()),
        None => Ok(min),
    }
}

fn calculate_row(input: &str) -> Result<usize> {
    calculate_row_rec(input, 0, 127)
}

fn calculate_column_rec(input: &str, min: usize, max: usize) -> Result<usize> {
    match input.chars().next() {
        Some(LEFT) => calculate_column_rec(&input[1..], min, min + (max - min) / 2),
        Some(RIGHT) => calculate_column_rec(&input[1..], min + 1 + (max - min) / 2, max),
        Some(_) => Err(DayError::FailedParsingPosition.into()),
        None => Ok(min),
    }
}

fn calculate_column(input: &str) -> Result<usize> {
    calculate_column_rec(input, 0, 7)
}

#[derive(Debug, Eq, PartialEq)]
struct SeatLocation {
    row: usize,
    column: usize,
    id: usize,
}

impl SeatLocation {
    fn parse(input: &str) -> Result<SeatLocation> {
        let row = calculate_row(input.get(0..7).ok_or(DayError::FailedParsingPosition)?)?;
        let column = calculate_column(input.get(7..10).ok_or(DayError::FailedParsingPosition)?)?;
        let id = row * 8 + column;
        Ok(SeatLocation { row, column, id })
    }
}

fn task_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| SeatLocation::parse(line))
        .filter_map(Result::ok)
        .map(|seat_location| seat_location.id)
        .max()
        .ok_or(DayError::FailedToFindSeat)?)
}

fn task_2(input: &str) -> Result<usize> {
    let seat_ids: HashSet<usize> = input
        .lines()
        .map(|line| SeatLocation::parse(line))
        .filter_map(Result::ok)
        .map(|seat_location| seat_location.id)
        .collect();
    let min = *seat_ids.iter().min().ok_or(DayError::FailedToFindSeat)?;
    let max = *seat_ids.iter().max().ok_or(DayError::FailedToFindSeat)?;
    for id in min..max {
        if !seat_ids.contains(&id) {
            return Ok(id);
        }
    }
    Err(DayError::FailedToFindSeat.into())
}

pub fn run() {
    println!("Day 5 task 1 -> {}", task_1(input()).unwrap());
    println!("Day 5 task 2 -> {}", task_2(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_row_1() {
        let row_info = "FBFBBFF";
        let row_id = calculate_row(row_info);
        assert_eq!(row_id.unwrap(), 44);
    }

    #[test]
    fn calculate_row_2() {
        let row_info = "BFFFBBF";
        let row_id = calculate_row(row_info);
        assert_eq!(row_id.unwrap(), 70);
    }

    #[test]
    fn calculate_column_1() {
        let column_info = "RRR";
        let column_id = calculate_column(column_info);
        assert_eq!(column_id.unwrap(), 7);
    }

    #[test]
    fn calculate_column_2() {
        let column_info = "RLL";
        let column_id = calculate_column(column_info);
        assert_eq!(column_id.unwrap(), 4);
    }

    #[test]
    fn parse_example_seat_1() {
        let seat_info = "BFFFBBFRRR";
        let seat = SeatLocation::parse(seat_info).unwrap();
        assert_eq!(
            seat,
            SeatLocation {
                row: 70,
                column: 7,
                id: 567
            }
        );
    }

    #[test]
    fn parse_example_seat_2() {
        let seat_info = "FFFBBBFRRR";
        let seat = SeatLocation::parse(seat_info).unwrap();
        assert_eq!(
            seat,
            SeatLocation {
                row: 14,
                column: 7,
                id: 119
            }
        );
    }

    #[test]
    fn parse_example_seat_3() {
        let seat_info = "BBFFBBFRLL";
        let seat = SeatLocation::parse(seat_info).unwrap();
        assert_eq!(
            seat,
            SeatLocation {
                row: 102,
                column: 4,
                id: 820
            }
        );
    }

    #[test]
    fn task_1_test() {
        let highest_id = task_1(input()).unwrap();
        assert_eq!(highest_id, 888);
    }

    #[test]
    fn task_2_test() {
        let highest_id = task_2(input()).unwrap();
        assert_eq!(highest_id, 522);
    }
}
