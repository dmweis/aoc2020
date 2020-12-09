use anyhow::Result;
use std::collections::{HashSet, VecDeque};

fn input() -> Vec<i64> {
    include_str!("inputs/day_9.txt")
        .lines()
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .unwrap()
}

struct FirstInEvictionSet {
    set: HashSet<i64>,
    order: VecDeque<i64>,
    max_capacity: usize,
}

impl FirstInEvictionSet {
    fn new(max_capacity: usize) -> Self {
        FirstInEvictionSet {
            set: HashSet::new(),
            order: VecDeque::new(),
            max_capacity,
        }
    }

    fn insert(&mut self, value: i64) {
        if self.order.len() > self.max_capacity {
            let value_to_remove = self.order.pop_back().unwrap();
            self.set.remove(&value_to_remove);
        }
        self.order.push_front(value);
        if !self.set.insert(value) {
            panic!("Set already contained value.\nValues repeat more than once in every 25.\nYou need a counting dict.")
        }
    }

    fn contains_valid_parts(&self, sum: i64) -> bool {
        for number in &self.order {
            let other = sum - number;
            if self.set.contains(&other) && number != &other {
                return true;
            }
        }
        false
    }
}

fn task_1(input: Vec<i64>) -> Option<i64> {
    let mut counter = FirstInEvictionSet::new(25);
    // preamble
    for number in input.iter().take(25) {
        counter.insert(*number);
    }
    for number in input.into_iter().skip(25) {
        if !counter.contains_valid_parts(number) {
            return Some(number);
        }
        counter.insert(number)
    }
    None
}

fn task_2(numbers: Vec<i64>, target: i64) -> Option<i64> {
    for (index, number) in numbers.iter().enumerate() {
        let mut collect = vec![number];
        for other in numbers.get(index + 1..)? {
            collect.push(other);
            let sum: i64 = collect.iter().cloned().sum();
            if sum == target {
                return Some(*collect.iter().max()? + *collect.iter().min()?);
            } else if sum > target {
                break;
            }
        }
    }
    None
}

pub fn run() {
    let task_1_res = task_1(input()).unwrap();
    println!("Day 9 task 1 -> {}", task_1_res);
    println!("Day 9 task 2 -> {}", task_2(input(), task_1_res).unwrap());
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use super::*;

    #[test]
    fn example_text() {
        let mut counter = FirstInEvictionSet::new(25);
        for number in 1..26 {
            counter.insert(number);
        }
        assert!(counter.contains_valid_parts(26));
        assert!(counter.contains_valid_parts(49));
        assert!(!counter.contains_valid_parts(100));
        assert!(!counter.contains_valid_parts(50));
    }

    #[test]
    #[should_panic]
    fn panics_on_same() {
        let mut counter = FirstInEvictionSet::new(25);
        for number in repeat(4) {
            counter.insert(number);
        }
    }

    #[test]
    fn exmple_1() {
        let example = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut counter = FirstInEvictionSet::new(5);
        // load preamble
        for number in example.iter().take(5) {
            counter.insert(*number);
        }
        // rest
        for number in example.iter().skip(5) {
            if *number == 127 {
                assert!(!counter.contains_valid_parts(*number));
            } else {
                assert!(counter.contains_valid_parts(*number));
            }
            counter.insert(*number);
        }
    }

    #[test]
    fn task_1_test() {
        let res = task_1(input()).unwrap();
        assert_eq!(res, 257342611);
    }

    #[test]
    fn task_2_test() {
        let res = task_2(input(), 257342611).unwrap();
        assert_eq!(res, 35602097);
    }

    #[test]
    fn task_2_example_1() {
        let example = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let res = task_2(example, 127).unwrap();
        assert_eq!(res, 62);
    }
}
