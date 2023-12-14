use core::fmt;
use itertools::Itertools;
use std::{collections::HashMap, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let seeds_text = lines.next().unwrap();
    let seeds = parse_seeds(seeds_text).unwrap().1;
    lines.next();
    let mut maps = HashMap::new();
    while let Some(map) = parse_map(&mut lines) {
        maps.insert(map.key_in, map);
    }

    println!("Part 1: {:?}", get_smallest_seed(seeds, &maps));
    // let seed_ranges = parse_seeds_range(seeds_text).unwrap().1;
    // println!("Part 2: {:?}", get_smallest_seed_range(seed_ranges, &maps));
}

fn get_smallest_seed(seeds: Vec<u64>, maps: &HashMap<Key, Map>) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            let mut current_key = Key::Seed;
            let mut num = *seed;
            while current_key != Key::Location {
                let map = maps.get(&current_key).unwrap();
                num = map.translate_number(num);
                current_key = map.key_out;
            }
            num
        })
        .min()
        .unwrap()
}

// fn get_smallest_seed_range(seed_ranges: HashMap<u64, Range<u64>>, maps: &HashMap<Key, Map>) -> u64 {
//     seed_ranges
//         .iter()
//         .map(|range| {
//             let x = 5;
//             get_smallest_seed(range.clone().collect(), maps)
//         })
//         .min()
//         .unwrap()
// }

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Key {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Seed => write!(f, "Seed"),
            Key::Soil => write!(f, "Soil"),
            Key::Fertilizer => write!(f, "Fertilizer"),
            Key::Water => write!(f, "Water"),
            Key::Light => write!(f, "Light"),
            Key::Temperature => write!(f, "Temperature"),
            Key::Humidity => write!(f, "Humidity"),
            Key::Location => write!(f, "Location"),
        }
    }
}

#[derive(Clone)]
struct Transformation {
    range: Range<u64>,
    transformation: i64,
}

impl Transformation {
    fn transform(&self, num: u64) -> Option<u64> {
        if self.range.contains(&num) {
            Some(((num as i64) + self.transformation) as u64)
        } else {
            None
        }
    }
}

impl fmt::Debug for Transformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}, {}",
            self.range.start, self.range.end, self.transformation
        )
    }
}

struct Map {
    transformations: Vec<Transformation>,
    key_in: Key,
    key_out: Key,
}

impl Map {
    fn translate_number(&self, num: u64) -> u64 {
        self.transformations
            .iter()
            .find_map(|transformation| transformation.transform(num))
            .unwrap_or(num)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} -> {:?}: {:?}",
            self.key_in, self.key_out, self.transformations
        )
    }
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("seeds:"),
        map(many1(preceded(tag(" "), digit1)), |n| {
            n.iter().map(|n: &&str| n.parse().unwrap()).collect()
        }),
    )(i)
}

fn parse_seeds_range(i: &str) -> IResult<&str, HashMap<u64, Range<u64>>> {
    preceded(
        tag("seeds:"),
        map(many1(preceded(tag(" "), digit1)), |n| {
            n.iter()
                .map(|n: &&str| n.parse::<u64>().unwrap())
                .tuples()
                .map(|(a, b)| (a, a..(a + b)))
                .collect()
        }),
    )(i)
}

fn parse_map<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Option<Map> {
    let (from, to) = all_consuming(parse_map_name)(lines.next()?)
        .finish()
        .ok()?
        .1;
    let transformations: Vec<Transformation> = lines
        .map_while(|line| {
            all_consuming(parse_transformation)(line)
                .finish()
                .ok()
                .map(|(_, keys)| keys)
        })
        .collect();

    Some(Map {
        transformations,
        key_in: from,
        key_out: to,
    })
}

fn parse_map_name(i: &str) -> IResult<&str, (Key, Key)> {
    terminated(
        separated_pair(parse_key, tag("-to-"), parse_key),
        tag(" map:"),
    )(i)
}

fn parse_key(i: &str) -> IResult<&str, Key> {
    alt((
        map(tag("seed"), |_| Key::Seed),
        map(tag("soil"), |_| Key::Soil),
        map(tag("fertilizer"), |_| Key::Fertilizer),
        map(tag("water"), |_| Key::Water),
        map(tag("light"), |_| Key::Light),
        map(tag("temperature"), |_| Key::Temperature),
        map(tag("humidity"), |_| Key::Humidity),
        map(tag("location"), |_| Key::Location),
    ))(i)
}

fn parse_transformation(i: &str) -> IResult<&str, Transformation> {
    let (i, dest_start) = terminated(digit1, tag(" "))(i)?;
    map(
        separated_pair(digit1, tag(" "), digit1),
        |(source_start, len)| calculate_transformation(dest_start, source_start, len),
    )(i)
}

fn calculate_transformation(dest_start: &str, source_start: &str, len: &str) -> Transformation {
    let start = source_start.trim().parse::<u64>().unwrap();
    let end = start + len.trim().parse::<u64>().unwrap();
    let dest_start = dest_start.trim().parse::<i64>().unwrap();
    let transformation = dest_start.checked_sub(start as i64).unwrap();
    Transformation {
        range: start..(end + 1),
        transformation,
    }
}
