use core::cmp::Ordering;
use itertools::Itertools;
use std::{fmt::Debug, str::FromStr};

/// Pair //////////////////////////////////////////////////////////////////////////////////////
pub struct Pair(Range, Range);

impl Pair {
    pub fn has_redundancy(&self) -> bool {
        match self.0.low.cmp(&self.1.low) {
            Ordering::Equal => true,
            Ordering::Greater => self.0.high <= self.1.high,
            Ordering::Less => self.0.high >= self.1.high,
        }
    }

    pub fn has_overlap(&self) -> bool {
        match self.0.low.cmp(&self.1.low) {
            Ordering::Equal => true,
            Ordering::Greater => self.0.low <= self.1.high,
            Ordering::Less => self.0.high >= self.1.low,
        }
    }
}

#[derive(Debug)]
pub struct ReadPairError;

impl FromStr for Pair {
    type Err = ReadPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (one, two) = s
            .split(',')
            .map(|v| Range::from_str(v).unwrap())
            .next_tuple()
            .unwrap();

        Ok(Pair(one, two))
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.0).field(&self.1).finish()
    }
}

/// Range //////////////////////////////////////////////////////////////////////////////////////
struct Range {
    low: u32,
    high: u32,
}

impl FromStr for Range {
    type Err = ReadPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s
            .split('-')
            .map(|v| v.to_string().parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();
        Ok(Range { low, high })
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.low)
            .field(&self.high)
            .finish()
    }
}
