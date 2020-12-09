use anyhow::Result;
use std::collections::HashSet;
use std::include_str;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DayError {
    #[error("MapNotFormatted properly. Either it's 0 in some dimension or not even")]
    MapNotFormatted,
    #[error("PointOutOfBounds")]
    PointOutOfBounds,
}

fn input() -> &'static str {
    include_str!("inputs/day_3.txt")
}

const TREE: char = '#';
const EMPTY: char = '.';

struct TreeMaze {
    trees: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl FromStr for TreeMaze {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut trees = HashSet::new();
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .ok_or(DayError::MapNotFormatted)?
            .chars()
            .count();
        if !input.lines().all(|line| line.chars().count() == width) {
            return Err(DayError::MapNotFormatted.into());
        }
        // Y is vertical with 0 at top
        // X is horizontal with 0 at left
        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                if character == TREE {
                    trees.insert((x, y));
                } else if character != EMPTY {
                    return Err(DayError::MapNotFormatted.into());
                }
            }
        }
        Ok(TreeMaze {
            trees,
            height,
            width,
        })
    }
}

impl TreeMaze {
    /// Check if location has a tree
    /// Will automatically handle width
    /// Y is vertical with 0 at top
    /// X is horizontal with 0 at left
    fn is_tree(&self, x: usize, y: usize) -> Result<bool> {
        let corrected_x = x % self.width;
        if y > self.height {
            return Err(DayError::PointOutOfBounds.into());
        }
        Ok(self.trees.contains(&(corrected_x, y)))
    }
}

struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Self {
        Slope { right, down }
    }
}

fn check_slope(maze: &TreeMaze, slope: &Slope) -> Result<usize> {
    let height = maze.height;
    let mut tree_hit_count = 0;
    for i in 0.. {
        let x = i * slope.right;
        let y = i * slope.down;
        if y > height {
            break;
        }
        if maze.is_tree(x, y)? {
            tree_hit_count += 1;
        }
    }
    Ok(tree_hit_count)
}

fn task_1(input: &str) -> Result<usize> {
    let maze = input.parse::<TreeMaze>()?;
    let tree_hit_count = check_slope(&maze, &Slope::new(3, 1))?;
    Ok(tree_hit_count)
}

fn task_2(input: &str) -> Result<usize> {
    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];
    let maze = input.parse::<TreeMaze>()?;
    let tree_hit_count = slopes
        .into_iter()
        .map(|slope| check_slope(&maze, &slope))
        .flat_map(Result::ok)
        .product();
    Ok(tree_hit_count)
}

pub fn run() {
    println!("Day 3 task 1 -> {}", task_1(input()).unwrap());
    println!("Day 3 task 2 -> {}", task_2(input()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_symbol() {
        let example = "ksjdfla";
        let maze = example.parse::<TreeMaze>();
        assert!(maze.is_err());
    }

    #[test]
    fn single_tree() {
        let example = "#";
        let maze = example.parse::<TreeMaze>().unwrap();
        assert!(maze.is_tree(0, 0).unwrap());
    }

    #[test]
    fn single_empty() {
        let example = ".";
        let maze = example.parse::<TreeMaze>().unwrap();
        assert!(!maze.is_tree(0, 0).unwrap());
    }

    #[test]
    fn single_square() {
        let example = ".#
.#";
        let maze = example.parse::<TreeMaze>().unwrap();
        assert!(!maze.is_tree(0, 0).unwrap());
        assert!(maze.is_tree(1, 0).unwrap());
        assert!(!maze.is_tree(0, 1).unwrap());
        assert!(maze.is_tree(1, 1).unwrap());
    }

    #[test]
    fn single_square_wrapping() {
        let example = ".#
.#";
        let maze = example.parse::<TreeMaze>().unwrap();
        assert!(maze.is_tree(3, 0).unwrap());
    }

    #[test]
    fn uneven_maze() {
        let example = ".#
.##";
        let maze = example.parse::<TreeMaze>();
        assert!(maze.is_err());
    }

    #[test]
    fn example_1_task_1() {
        let example = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let tree_hit_count = task_1(example).unwrap();
        assert_eq!(tree_hit_count, 7);
    }

    #[test]
    fn task_1_test() {
        let tree_hit_count = task_1(input()).unwrap();
        assert_eq!(tree_hit_count, 299);
    }

    #[test]
    fn example_1_task_2() {
        let example = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let tree_hit_count = task_2(example).unwrap();
        assert_eq!(tree_hit_count, 336);
    }

    #[test]
    fn task_2_test() {
        let tree_hit_count = task_2(input()).unwrap();
        assert_eq!(tree_hit_count, 3621285278);
    }
}
