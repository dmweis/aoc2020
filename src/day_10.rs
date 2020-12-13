use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn input() -> &'static str {
    include_str!("inputs/day_10.txt")
}

fn task_1(input: &str) -> Result<i32> {
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(Result::ok)
        .collect::<HashSet<_>>();
    let mut current_joltage = 0;
    let mut diff_table = HashMap::new();
    diff_table.insert(1, 0);
    diff_table.insert(2, 0);
    diff_table.insert(3, 1);

    while !adapters.is_empty() {
        for jump in 1..=3 {
            let target = current_joltage + jump;
            if adapters.take(&target).is_some() {
                *diff_table.get_mut(&jump).unwrap() += 1;
                current_joltage = target;
                break;
            }
        }
    }
    Ok(diff_table.get(&1).unwrap() * diff_table.get(&3).unwrap())
}

pub fn run() {
    println!("Day 10 task 1 -> {}", task_1(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_example_1() {
        let example = "16
10
15
5
1
11
7
19
6
12
4";
        let res = task_1(&example).unwrap();
        assert_eq!(res, 7 * 5);
    }

    #[test]
    fn task_1_example_2() {
        let example = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let res = task_1(&example).unwrap();
        assert_eq!(res, 22 * 10);
    }

    #[test]
    fn task_1_test() {
        let res = task_1(input()).unwrap();
        assert_eq!(res, 2201)
    }
}
