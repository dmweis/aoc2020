use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"\s*(?P<number>\d*)\s(?P<color_name>[[:alpha:]\s]*)( bag| bags)").unwrap();
}

fn input() -> &'static str {
    include_str!("inputs/day_7.txt")
}

#[derive(Error, Debug)]
pub enum DayError {
    #[error("FailedParsingRule")]
    FailedParsingRule,
}

const MY_BAG: &str = "shiny gold";

#[derive(Debug, Eq, PartialEq)]
struct ContainedBag {
    color: String,
    count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct BagRule {
    parent: String,
    children: Vec<ContainedBag>,
}

impl BagRule {
    fn parse(input: &str) -> Result<BagRule> {
        let mut split_halves = input.split(" bags contain ");
        let parent_name = split_halves.next().ok_or(DayError::FailedParsingRule)?;

        let child_list = split_halves.next().ok_or(DayError::FailedParsingRule)?;
        let child_text_fields = if child_list.contains("no other bags.") {
            Ok(vec![])
        } else {
            child_list
                .split(|character| char::is_ascii_punctuation(&character))
                .filter(|line| !line.is_empty())
                .map(|contents| {
                    let captures = RULE_REGEX
                        .captures(contents)
                        .ok_or(DayError::FailedParsingRule)?;
                    let color = captures
                        .name("color_name")
                        .ok_or(DayError::FailedParsingRule)?
                        .as_str()
                        .to_owned();
                    let count = captures
                        .name("number")
                        .ok_or(DayError::FailedParsingRule)?
                        .as_str()
                        .parse()?;
                    Ok(ContainedBag { color, count })
                })
                .collect::<Result<Vec<_>>>()
        };
        Ok(BagRule {
            parent: parent_name.to_owned(),
            children: child_text_fields?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RuleSet {
    bag_to_containers: HashMap<String, Vec<String>>,
}

impl RuleSet {
    fn parse_ruleset(input: &str) -> Result<RuleSet> {
        let rules = input.lines().map(BagRule::parse).filter_map(Result::ok);
        let mut bag_to_containers = HashMap::new();
        for rule in rules {
            for child in rule.children {
                let containers = bag_to_containers.entry(child.color).or_insert(vec![]);
                containers.push(rule.parent.clone());
            }
        }
        Ok(RuleSet { bag_to_containers })
    }

    fn count_topmost_containers(&self) -> usize {
        let start = MY_BAG;
        let mut top_most_colors = HashSet::new();
        let mut seen = VecDeque::new();
        seen.push_front(start);
        while let Some(node) = seen.pop_back() {
            if let Some(children) = self.bag_to_containers.get(node) {
                for child in children {
                    seen.push_front(child);
                }
            }
            top_most_colors.insert(node);
        }
        // remove starting color
        top_most_colors.len() - 1
    }
}

fn task_1(input: &str) -> Result<usize> {
    let rule_set = RuleSet::parse_ruleset(input)?;
    Ok(rule_set.count_topmost_containers())
}

pub fn run() {
    println!("Day 7 task 1 -> {}", task_1(input()).unwrap());
    // println!("Day 7 task 2 -> {}", task_2(input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let rule_set = RuleSet::parse_ruleset(example).unwrap();
        assert_eq!(rule_set.count_topmost_containers(), 4);
    }

    #[test]
    fn parse_ruleset_example() {
        let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.";
        let rule_set = RuleSet::parse_ruleset(&example).unwrap();
        assert!(rule_set
            .bag_to_containers
            .get("muted yellow")
            .unwrap()
            .contains(&"light red".to_owned()));
        assert!(rule_set
            .bag_to_containers
            .get("bright white")
            .unwrap()
            .contains(&"light red".to_owned()));
        assert!(rule_set
            .bag_to_containers
            .get("bright white")
            .unwrap()
            .contains(&"dark orange".to_owned()));
        assert!(rule_set
            .bag_to_containers
            .get("muted yellow")
            .unwrap()
            .contains(&"dark orange".to_owned()));
    }

    #[test]
    fn parse_rule_1() {
        let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let res = BagRule::parse(&example).unwrap();
        let expected = BagRule {
            parent: "light red".to_owned(),
            children: vec![
                ContainedBag {
                    color: "bright white".to_owned(),
                    count: 1,
                },
                ContainedBag {
                    color: "muted yellow".to_owned(),
                    count: 2,
                },
            ],
        };
        assert_eq!(expected, res);
    }

    #[test]
    fn parse_rule_no_other() {
        let example = "faded blue bags contain no other bags.";
        let res = BagRule::parse(&example).unwrap();
        let expected = BagRule {
            parent: "faded blue".to_owned(),
            children: vec![],
        };
        assert_eq!(expected, res);
    }

    #[test]
    fn task_1_test() {
        let res = task_1(input()).unwrap();
        assert!(res < 204);
        assert_eq!(res, 197);
    }

    #[test]
    fn rule_capture() {
        let rule = "1 bright white bag";
        let captures = RULE_REGEX.captures(rule).unwrap();
        assert_eq!(captures.name("number").unwrap().as_str(), "1");
        assert_eq!(
            captures.name("color_name").unwrap().as_str(),
            "bright white"
        );
    }

    #[test]
    fn rule_capture_white_space() {
        let rule = " 1 bright white bag";
        let captures = RULE_REGEX.captures(rule).unwrap();
        assert_eq!(captures.name("number").unwrap().as_str(), "1");
        assert_eq!(
            captures.name("color_name").unwrap().as_str(),
            "bright white"
        );
    }
}
