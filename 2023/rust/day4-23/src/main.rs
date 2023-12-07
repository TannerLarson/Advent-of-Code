use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{delimited, pair, preceded, separated_pair},
    Finish, IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let cards: Vec<_> = input
        .lines()
        .map(|line| {
            all_consuming(parse_card)(line)
                .finish()
                .ok()
                .map(|(_, card)| card)
                .unwrap()
        })
        .collect();
    let part_1: u32 = cards
        .iter()
        .map(|card| {
            let n = card.num_matches;
            if n == 0 {
                0
            } else {
                2_u32.pow(n - 1)
            }
        })
        .sum();
    println!("Part 1: {:?}", part_1);

    let id_map: HashMap<u32, Card> = cards.iter().map(|card| (card.id, card.clone())).collect();
    let num_copies: u32 = (1..(cards.len() + 1))
        .map(|card_id| {
            // println!("Card ID: {}", card_id);
            get_copies(card_id as u32, &id_map).len() as u32
        })
        .sum();
    println!("Part 2: {:?}", num_copies);
}

fn get_copies(id: u32, id_map: &HashMap<u32, Card>) -> Vec<u32> {
    let mut stack = vec![id];
    let mut copies: Vec<u32> = Vec::new();
    while let Some(id) = stack.pop() {
        copies.push(id);
        let mut copy_ids = id_map.get(&id).unwrap().get_copy_ids();
        // println!("{}: {:?}", id, copy_ids);
        if !copy_ids.is_empty() {
            stack.append(&mut copy_ids);
        }
    }
    copies
}

#[derive(Clone)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
    num_matches: u32,
}

impl Card {
    fn get_copy_ids(&self) -> Vec<u32> {
        (1..(self.num_matches + 1)).map(|i| self.id + i).collect()
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?} | {:?}", self.id, self.winning, self.have)
    }
}

fn parse_id(i: &str) -> IResult<&str, u32> {
    map(
        delimited(
            pair(tag("Card"), take_while(|c: char| c.is_whitespace())),
            digit1,
            tag(":"),
        ),
        |num: &str| num.parse::<u32>().unwrap(),
    )(i)
}

fn parse_number(i: &str) -> IResult<&str, u32> {
    map(preceded(alt((tag("  "), tag(" "))), digit1), |num: &str| {
        num.parse::<u32>().unwrap()
    })(i)
}

fn parse_numbers(i: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    separated_pair(
        map(many1(parse_number), HashSet::from_iter),
        tag(" |"),
        map(many1(parse_number), HashSet::from_iter),
    )(i)
}

fn parse_card(i: &str) -> IResult<&str, Card> {
    map(pair(parse_id, parse_numbers), |(id, (winning, have))| {
        let num_matches = winning.intersection(&have).copied().count() as u32;
        Card {
            id,
            winning,
            have,
            num_matches,
        }
    })(i)
}
