use core::fmt;
use itertools::Itertools;
use num_integer::binomial;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::thread::current;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let input: Vec<Springs> = include_str!("ex2.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    for springs in input {
        println!("{:?}, {:?}", springs.line, springs.damaged);
        springs.possible_lines();
        println!()
    }
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

    /*
        ?#??#?##??.??? 7,1,1
        sum_damaged = self.len
        n = 5
        k = 2, 3, 4

        k = 4
        [2 1 1 1]
        [1 2 1 1]
        [1 1 2 1]
        [1 1 1 2]

        k = 3
        - [2 1 1] - 3
        [3 1 1]
        [2 2 1]
        [2 1 2]
        - [1 2 1] - 2
        [1 3 1]
        [1 2 2]
        - [1 1 2] - 1
        [1 1 3]
        
        k = 2
        --- [1 1] ---
        -- [2 1] --
        - [3 1] -
        [4 1]
        [3 2]
        - [2 2] -

    */
    fn possible_lines(&self) {
        let num_stars = self.len - (self.damaged.iter().sum::<u8>() as usize + self.damaged.len() - 1);
        let num_buckets = self.len - self.damaged.len();

        // Vector to keep the results
        let lines: HashSet<Line> = HashSet::new();

        // Initialize stack with a bunch of vectors ([1 0 0], [0 1 0], [0 0 1])
        let mut stack: Vec<Vec<u8>> = vec![vec![0; num_buckets]];

        while let Some(operational) = stack.pop() {
            // Base case
            if operational.iter().sum::<u8>() == num_stars as u8 {
                // Convert config into string
                // Add string to `lines` hash set
                /// ??????   1, 1, 1
                let first_condition = match operational.len().cmp(*self.damaged.len()) {
                    Ordering::Greater => Condition::Operational,
                    Ordering::Less => Condition::Operational,
                    Ordering::Equal => 
                }
                lines.insert(Line::from_spring_data(
                    self.damaged,
                    operational,
                    first_condition,
                ))
            } else {
                // For loop from 0 to num_buckets
                // Increment config[i] by 1 and push a copy on the stack
            }
        }

        let base_line = self
            .damaged
            .iter()
            .map(|&num| "#".repeat(num as usize))
            .collect::<Vec<String>>()
            .join("|");
        let insert_indexes: Vec<usize> = std::iter::once(0)
            .chain(
                base_line
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '|').then_some(i)),
            )
            .chain(std::iter::once(base_line.len()))
            .collect();
        println!("base: {}, indexes: {:?}", base_line, insert_indexes);

        let mut current_line = base_line;
        let mut possible_lines: Vec<String> = Vec::new();
        for (i, insert_index) in insert_indexes.iter().enumerate() {
            // Start by putting all our stars at the current insert index
            current_line.insert_str(*insert_index, &"*".repeat(num_stars));
            possible_lines.push(current_line.clone());
            for _ in 0..num_stars {
                current_line.remove(*insert_index);
                for j in insert_indexes.iter().skip(i) {
                    current_line.insert(*j, '.');
                    println!("{:?}", current_line);
                    possible_lines.push(current_line.clone());
                    current_line.remove(*j);
                }
                println!()
            }
        }
        println!(
            "{:?}",
            self.damaged.iter().sum::<u8>() as usize + self.damaged.len()
        );
        println!("len: {:?}", self.len);
        println!("hint: {:?}", self.damaged);
        println!("num_stars: {}", num_stars);
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

#[derive(Clone, Copy)]
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
