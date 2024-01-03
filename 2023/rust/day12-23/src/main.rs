use core::fmt;
use itertools::Itertools;
use num_integer::binomial;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let input: Vec<Springs> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // for springs in input {
    //     println!("{:?}, {:?}", springs.line, springs.damaged);
    //     springs.all_possible_lines();
    //     println!()
    // }
    println!(
        "Part 1: {}",
        input
            .iter()
            .map(|springs| springs.all_possible_lines().len() as u32)
            .sum::<u32>()
    )
}

struct Springs {
    line: Line,
    damaged: Vec<u8>,
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

        // Initialize stack with a bunch of vectors ([1 1 1], [1 1 1], [1 1 1])
        let mut stack: Vec<Vec<u8>> = vec![vec![1; num_buckets]];
        // println!("Stars: {:?}", num_stars);
        // println!("Buckets: {:?}\n", num_buckets);
        // println!("Stack: {:?}", stack);
        // println!("Lines: {:?}", lines);

        while let Some(operational) = stack.pop() {
            // Base case
            if operational.iter().sum::<u8>() == num_stars as u8 {
                match operational.len().cmp(&self.damaged.len()) {
                    Ordering::Greater => {
                        // println!("Operational: {:?}", operational);
                        lines.insert(Line::from_spring_data(
                            self.damaged.clone(),
                            operational,
                            Condition::Operational,
                        ));
                    }
                    Ordering::Less => {
                        // println!("Operational: {:?}", operational);
                        lines.insert(Line::from_spring_data(
                            self.damaged.clone(),
                            operational,
                            Condition::Damaged,
                        ));
                    }
                    Ordering::Equal => {
                        // println!("Operational: {:?}", operational);
                        lines.insert(Line::from_spring_data(
                            self.damaged.clone(),
                            operational.clone(),
                            Condition::Operational,
                        ));
                        lines.insert(Line::from_spring_data(
                            self.damaged.clone(),
                            operational,
                            Condition::Damaged,
                        ));
                    }
                }
            } else {
                for i in 0..num_buckets {
                    let mut new_op = operational.clone();
                    new_op[i] += 1;
                    stack.push(new_op);
                }
            }
            // println!("Stack: {:?}", stack);
            // println!("Lines: {:?}", lines);
        }
        lines
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
        let hint: Vec<u8> = iter
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
    /// Convert damaged and operational data into a combined line
    /// damaged: [8,1,2,1]
    /// operational: [3 1 1 1]
    fn from_spring_data(
        damaged: Vec<u8>,
        operational: Vec<u8>,
        first_condition: Condition,
    ) -> Self {
        let (mut first, mut second) = match first_condition {
            Condition::Damaged => (VecDeque::from(damaged), VecDeque::from(operational)),
            Condition::Operational => (VecDeque::from(operational), VecDeque::from(damaged)),
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
/*
1, 3, 2: goal
...##??: actual

1. Compare minimum goal size to actual size

We can guarantee actuals based on the difference between goal size and actual size. If the difference is x, we can guarantee y-x actuals where y is the goal number.

2. Fill out guaranteed squares

To find specifically which ones to fill out, start with a for loop over the goal numbers. Also keep track of the current actual index, with the index starting at -1.
1. Increase the index by the current goal number
2. Starting at the current index, fill out squares equal to the difference found previously backwards. So, if the current index is 6 and the difference is 2, fill out squares 6 and 5.
3. Add one to the index to account for a single space in between the goal numbers

3. Extrapolate current possibilities

---------------------------------------------------------------------------------------------

STARS AND BARS METHOD

| = Required blank
* = Optional blank
x = actual

Assume a 15 x 15 grid

3, 8
xxx|xxxxxxxx

***xxx|xxxxxxxx
xxx***|xxxxxxxx / xxx|***xxxxxxxx
xxx|xxxxxxxx***

**xxx*|xxxxxxxx / **xxx|*xxxxxxxx
**xxx|xxxxxxxx*

*xxx**|xxxxxxxx / *xxx|**xxxxxxxx
*xxx|xxxxxxxx**

n = The number of additional gap cells (aka stars) we can distribute = 3
k = The number of positions where we can place additional gap cells = 3


??#?##????#???..? 8,1,2,1
n = 2
k = 5
xxxxxxxx|x|xx|x

**xxxxxxxx|x|xx|x
*xxxxxxxx*|x|xx|x
*xxxxxxxx|x*|xx|x
*xxxxxxxx|x|xx*|x
*xxxxxxxx|x|xx|x*
xxxxxxxx**|x|xx|x
xxxxxxxx*|x*|xx|x
xxxxxxxx*|x|xx*|x
xxxxxxxx*|x|xx|x*
xxxxxxxx|x**|xx|x
xxxxxxxx|x*|xx*|x
xxxxxxxx|x*|xx|x*
xxxxxxxx|x|xx**|x
xxxxxxxx|x|xx*|x*
xxxxxxxx|x|xx|x**
*/
