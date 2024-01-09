use core::fmt;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;

// Last input run time: 2.37s

fn main() {
    // let now = Instant::now();
    let mut input: Vec<Springs> = include_str!("ex2.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // println!(
    //     "Part 1: {}",
    //     input
    //         .iter()
    //         .map(|springs| springs.all_possible_lines().len() as u32)
    //         .sum::<u32>()
    // );
    input = input.iter().map(|springs| springs.unfolded()).collect();
    println!("Unfolded: {:?}", input);
    let ans = input
        .iter()
        .map(|springs| springs.all_possible_lines().len() as u32)
        .sum::<u32>();
    println!("Part 2: {}", ans)

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed)
}

struct Springs {
    line: Line,
    damaged: Vec<u8>,
    len: usize,
}

impl Springs {
    // "Unfold" input according to part 2
    fn unfolded(&self) -> Self {
        let mut l = self.line.0.clone();
        l.push(Condition::Unknown);
        let mut line = Line(std::iter::repeat(l).take(5).flatten().collect_vec());
        line.0.pop();

        let damaged = std::iter::repeat(self.damaged.clone())
            .take(5)
            .flatten()
            .collect_vec();
        let len = self.len * 5 + 4;
        Self { line, damaged, len }
    }

    /// Stars and Bars method
    /// n = Things I want to distribute
    /// k = Buckets to distribute across
    /// If hint == [3, 8] and len == 15, then n = 3 (15 - 12) and k = 3 (before 3, between 3 and 8, after 8)
    // fn theoretical_num_configs(&self) -> u8 {
    //     let n = self.len as u8 - (self.damaged.iter().sum::<u8>() + self.damaged.len() as u8 - 1);
    //     let k = (self.len + 1) as u8;
    //     binomial(n - 1, k)
    // }

    /// Account for all possible number of buckets
    fn all_possible_lines(&self) -> Vec<Line> {
        let num_stars = self.len - self.damaged.iter().sum::<u8>() as usize;
        let min_num_buckets = self.damaged.len() - 1;
        let mut lines = self.possible_lines(num_stars, min_num_buckets);
        if num_stars > min_num_buckets {
            lines.extend(self.possible_lines(num_stars, min_num_buckets + 1))
        }
        if num_stars > min_num_buckets + 1 {
            lines.extend(self.possible_lines(num_stars, min_num_buckets + 2))
        }
        // lines.iter().for_each(|line| println!("{:?}", line));
        lines
    }

    // Get all possible lines for a specific number of buckets and stars
    fn possible_lines(&self, num_stars: usize, num_buckets: usize) -> Vec<Line> {
        assert!(
            num_stars >= num_buckets,
            "num_stars ({}) must be >= num_buckets ({})",
            num_stars,
            num_buckets
        );
        // Cache key should be input, value should be output
        let mut cache: HashMap<Vec<u8>, Vec<Line>> = HashMap::new();
        // stack should have a bunch of inputs
        let mut stack: Vec<Vec<u8>> = Vec::new();

        let starting_conf = vec![1; num_buckets];
        stack.push(starting_conf.clone());

        while let Some(conf) = stack.pop() {
            // println!("Stack: {:?} + {:?}", stack, conf);
            // println!("Cache: {:?}\n", cache.keys());
            if cache.contains_key(&conf) {
                continue;
            }
            // Base case
            if conf.iter().sum::<u8>() == num_stars as u8 {
                let mut new_lines = Vec::new();
                if conf.len() >= self.damaged.len() {
                    new_lines.push(self.build_line(&conf, Condition::Operational))
                }
                if conf.len() <= self.damaged.len() {
                    new_lines.push(self.build_line(&conf, Condition::Damaged))
                }
                cache.insert(conf.clone(), new_lines.into_iter().flatten().collect());
                continue;
            }
            let branches = (0..num_buckets).map(|i| {
                let mut new: Vec<u8> = Vec::new();
                new.extend(conf.clone());
                new[i] += 1;
                new
            });
            if branches.clone().all(|branch| cache.contains_key(&branch)) {
                let lines = branches
                    .flat_map(|branch| cache.get(&branch).unwrap())
                    .cloned()
                    .collect_vec();
                cache.insert(conf, lines);
            } else {
                stack.push(conf.clone());
                branches.for_each(|branch| stack.push(branch));
            }
        }

        cache
            .get(&starting_conf)
            .unwrap()
            .iter()
            .unique()
            .cloned()
            .collect()
    }

    // Builds a Line object
    fn build_line(&self, operational: &[u8], first_condition: Condition) -> Option<Line> {
        let (it_first, it_second) = match first_condition {
            Condition::Damaged => (self.damaged.iter(), operational.iter()),
            Condition::Operational => (operational.iter(), self.damaged.iter()),
            Condition::Unknown => panic!("Unknown is an incompatible condition type"),
        };
        // let mut conditions: Vec<Condition> = Vec::new();
        let mut current_condition = first_condition;
        let mut cond_it = self.line.0.iter();
        let mut line_data = Vec::new();

        for n in it_first.interleave(it_second) {
            // Make sure line is valid
            for _ in 0..*n {
                let cond = cond_it.next();
                if cond.is_none() || cond.unwrap() == &current_condition.opposite() {
                    return None;
                }
            }
            line_data.extend(vec![current_condition; *n as usize]);
            current_condition = current_condition.opposite()
        }
        Some(Line(line_data))
    }

    // fn is_line_valid(&self, line: &Line) -> bool {
    //     line.0
    //         .iter()
    //         .zip(self.line.0.iter())
    //         .all(|(a, b)| b == &Condition::Unknown || a == b)
    // }
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
