use std::collections::HashSet;

fn input() -> &'static str {
    include_str!("inputs/day_6.txt")
}

fn task_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|character| character.is_ascii_alphabetic())
                .collect::<HashSet<char>>()
        })
        .map(|group_answers| group_answers.len())
        .sum()
}

fn task_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.chars()
                        .filter(|character| character.is_ascii_alphabetic())
                        .collect::<HashSet<char>>()
                })
                .fold(
                    // TODO: replace with https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold_first once it becomes stable
                    ('a'..='z').collect::<HashSet<char>>(),
                    |acc, person| acc.intersection(&person).cloned().collect(),
                )
        })
        .map(|group_answers| group_answers.len())
        .sum()
}

pub fn run() {
    println!("Day 6 task 1 -> {}", task_1(input()));
    println!("Day 6 task 2 -> {}", task_2(input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_example_1() {
        let example = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let count = task_1(example);
        assert_eq!(count, 11);
    }

    #[test]
    fn task_1_test() {
        let count = task_1(input());
        assert_eq!(count, 6587);
    }

    #[test]
    fn task_2_example_1() {
        let example = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let count = task_2(example);
        assert_eq!(count, 6);
    }

    #[test]
    fn task_2_test() {
        let count = task_2(input());
        // original wrong answer
        assert_ne!(count, 3126);
        // number is bigger
        assert!(count > 3126);
        // I could remove the old asserts but I like that this keeps history
        assert_eq!(count, 3235);
    }
}
