use std::{collections::HashMap, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

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

struct Map {
    from: Key,
    to: Key,
    map: HashMap<Key, Key>,
}

fn main() {
    let lines = include_str!("./ex1.txt").lines();
    lines.for_each(|line| println!("{}", line))
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u32>> {
    let parse_numbers = |i: &str| preceded(tag(" "), digit1)(i);
    preceded(
        tag("seeds:"),
        map(many1(parse_numbers), |n| {
            n.iter().map(|n| n.parse().unwrap()).collect()
        }),
    )(i)
}

fn parse_map(i: &str) -> IResult<&str, Map> {
    // See day5-22 L189-200 for an idea of how to do this`
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

fn parse_key_range(i: &str) -> IResult<&str, (Range<u32>, Range<u32>)> {
    let (i, dest_start) = terminated(digit1, tag(" "))(i)?;
    map(
        separated_pair(digit1, tag(" "), digit1),
        |(source_start, len)| calculate_ranges(dest_start, source_start, len),
    )(i)
}

fn calculate_ranges(dest_start: &str, source_start: &str, len: &str) -> (Range<u32>, Range<u32>) {
    let dest_start = dest_start.parse::<u32>().unwrap();
    let source_start = source_start.parse::<u32>().unwrap();
    let len = len.parse::<u32>().unwrap();
    (
        dest_start..(dest_start + len),
        source_start..(source_start + len),
    )
}
