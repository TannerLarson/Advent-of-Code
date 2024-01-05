use core::fmt;
use itertools::Itertools;
use num_integer::binomial;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::time::Instant;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let now = Instant::now();
    let input: Vec<Springs> = include_str!("ex1.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    println!(
        "Part 1: {}",
        input
            .iter()
            .map(|springs| springs.all_possible_lines().len() as u32)
            .sum::<u32>()
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

struct Springs {
    line: Line,
    damaged: VecDeque<u8>,
    len: usize,
}

impl Springs {
    /// Stars and Bars method
    /// n = Things I want to distribute
    /// k = Buckets to distribute across
    /// If hint == [3, 8] and len == 15, then n = 3 (15 - 12) and k = 3 (before 3, between 3 and 8, after 8)
    fn theoretical_num_configs(&self) -> u8 {
        let n = self.len as u8 - (self.damaged.iter().sum::<u8>() + self.damaged.len() as u8 - 1);
        let k = (self.len + 1) as u8;
        binomial(n - 1, k)
    }

    /// Account for all possible number of buckets
    fn all_possible_lines(&self) -> HashSet<Line> {
        let num_stars = self.len - self.damaged.iter().sum::<u8>() as usize;
        let min_num_buckets = self.damaged.len() - 1;
        let mut lines = self.possible_lines(num_stars, min_num_buckets);
        if num_stars > min_num_buckets {
            lines.extend(self.possible_lines(num_stars, min_num_buckets + 1))
        }
        if num_stars > min_num_buckets + 1 {
            lines.extend(self.possible_lines(num_stars, min_num_buckets + 2))
        }
        lines
            .iter()
            .filter(|line| {
                line.0
                    .iter()
                    .zip(self.line.0.iter())
                    .all(|(con_l, con_r)| con_r == &Condition::Unknown || con_l == con_r)
            })
            .cloned()
            .collect()
    }

    // Get all possible lines for a specific number of buckets and stars
    fn possible_lines(&self, num_stars: usize, num_buckets: usize) -> HashSet<Line> {
        assert!(
            num_stars >= num_buckets,
            "num_stars ({}) must be >= num_buckets ({})",
            num_stars,
            num_buckets
        );
        // Vector to keep the results
        let mut lines: HashSet<Line> = HashSet::new();

        // Initialize stack with single starting vector [[1 1 1]]
        let mut stack: Vec<VecDeque<u8>> = vec![VecDeque::from(vec![1; num_buckets])];

        while let Some(operational) = stack.pop() {
            // Base case
            if operational.iter().sum::<u8>() == num_stars as u8 {
                match operational.len().cmp(&self.damaged.len()) {
                    Ordering::Greater => {
                        lines.insert(self.build_line(operational, Condition::Operational));
                    }
                    Ordering::Less => {
                        lines.insert(self.build_line(operational, Condition::Damaged));
                    }
                    Ordering::Equal => {
                        lines.insert(self.build_line(operational.clone(), Condition::Operational));
                        lines.insert(self.build_line(operational, Condition::Damaged));
                    }
                }
            } else {
                for i in 0..num_buckets {
                    let mut new_op = operational.clone();
                    new_op[i] += 1;
                    stack.push(new_op);
                }
            }
        }
        lines
    }

    fn build_line(&self, operational: VecDeque<u8>, first_condition: Condition) -> Line {
        // TODO: Change this so that self.damaged isn't cloned
        Line::from_spring_data(self.damaged.clone(), operational, first_condition)
    }

    fn is_line_valid(&self, line: Line) -> bool {
        let line_matches_spring_line = line
            .0
            .iter()
            .zip(self.line.0.iter())
            .all(|(a, b)| b == &Condition::Unknown || a == b);
        if !line_matches_spring_line {
            return false;
        }
        let mut damaged_it = self.damaged.iter();
        let mut count: Option<u8> = None;
        for cond in line.0 {
            if count.is_none() && cond == Condition::Damaged {
                if let Some(new_count) = damaged_it.next() {
                    count = Some(*new_count)
                } else {
                    return false;
                }
            }
            if count.is_some() {
                if count == Some(0) {
                    count = None;
                } else if cond == Condition::Operational {
                    return false;
                } else {
                    count = count.map(|n| n - 1);
                }
            }
            // println!("Cond: {:?}", cond);
            // println!("Count: {:?}\n", count);
        }
        damaged_it.next().is_none()
    }
}

#[derive(Debug)]
struct ParseSpringsError;

impl FromStr for Springs {
    type Err = ParseSpringsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.split_whitespace().next().unwrap().len();
        let mut iter = s.split_whitespace();
        let line = Line(
            iter.next()
                .unwrap()
                .chars()
                .map(|c| Condition::try_from(c).unwrap())
                .collect_vec(),
        );
        let hint: VecDeque<u8> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Ok(Springs {
            line,
            damaged: hint,
            len,
        })
    }
}

impl fmt::Debug for Springs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.line, self.damaged)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn opposite(&self) -> Self {
        match self {
            Condition::Operational => Condition::Damaged,
            Condition::Damaged => Condition::Operational,
            Condition::Unknown => panic!("Unknown doesn't have an opposite"),
        }
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

impl From<Condition> for char {
    fn from(value: Condition) -> Self {
        match value {
            Condition::Damaged => '#',
            Condition::Operational => '.',
            Condition::Unknown => '?',
        }
    }
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct Line(Vec<Condition>);

impl Line {
    fn from_spring_data(
        damaged: VecDeque<u8>,
        operational: VecDeque<u8>,
        first_condition: Condition,
    ) -> Self {
        let (mut first, mut second) = match first_condition {
            Condition::Damaged => (damaged, operational),
            Condition::Operational => (operational, damaged),
            Condition::Unknown => panic!("Unknown is an incompatible condition type"),
        };
        let mut conditions: Vec<Condition> = Vec::new();
        let mut current_condition = first_condition;

        while !first.is_empty() {
            conditions.extend((0..first.pop_front().unwrap()).map(|_| current_condition));
            current_condition = current_condition.opposite();
            if second.is_empty() {
                break;
            } else {
                conditions.extend((0..second.pop_front().unwrap()).map(|_| current_condition));
                current_condition = current_condition.opposite();
            }
        }
        Self(conditions)
    }
}

#[derive(Debug)]
struct ParseLineError;

impl FromStr for Line {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| c.try_into().unwrap()).collect_vec()))
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_string: String = self
            .0
            .iter()
            .map(|&con| {
                let c: char = con.into();
                c
            })
            .collect();
        write!(f, "{}", line_string)
    }
}
