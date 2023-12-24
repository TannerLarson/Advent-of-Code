use core::fmt;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let input: Vec<Springs> = include_str!("ex1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    for springs in input {
        println!("{:?}", springs);
    }
}

struct Springs {
    groups: Vec<Group>,
    group_sizes: Vec<u8>,
    len: u8
}

#[derive(Debug)]
struct ParseSpringsError;

impl FromStr for Springs {
    type Err = ParseSpringsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let mut iter = s.split_whitespace();
        let groups: Vec<Group> = iter
            .next()
            .unwrap()
            .chars()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, group)| {
                let s = group.collect::<String>();
                let condition: Condition = s.chars().next().unwrap().try_into().unwrap();
                let size = s.len();
                Group { condition, size }
            })
            .collect();
        let group_sizes: Vec<u8> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Ok(Springs {
            groups,
            group_sizes,
            len
        })
    }
}

impl fmt::Debug for Springs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.groups, self.group_sizes)
    }
}

enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Debug for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Condition::Unknown => '?',
            Condition::Operational => '.',
            Condition::Damaged => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct ParseConditionError;

impl TryFrom<char> for Condition {
    type Error = ParseConditionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Condition::Unknown),
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            _ => panic!("Tried to parse bad character: {value}"),
        }
    }
}

struct Group {
    condition: Condition,
    size: usize,
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.condition, self.size)
    }
}
/// Picross rules:
/// 1. Compare projected size against actual size. If it's equal, just fill in the squares
///     For example, if I have ????? (2, 2), that means it is ##.##
/// 2. If projected size is 1 less than actual size, we can guaruntee anything that is 2 or more
/// 3. If projected size is 2 less than the actual size, we can guaruntee anything that is 3 or more
