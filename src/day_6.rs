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

pub fn run() {
    println!("Day 6 task 1 -> {}", task_1(input()));
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
}
