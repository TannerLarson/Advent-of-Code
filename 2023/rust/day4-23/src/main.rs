use std::{collections::HashSet, fmt};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::digit1,
    character::is_space,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{delimited, pair, preceded, separated_pair},
    Finish, IResult,
};

struct Card {
    id: u32,
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn winning_numbers(&self) -> HashSet<u32> {
        self.winning.intersection(&self.have).copied().collect()
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
        Card { id, winning, have }
    })(i)
}

fn main() {
    let input = include_str!("input.txt");
    let cards: Vec<_> = input
        .lines()
        .map(|line| {
            let x = all_consuming(parse_card)(line)
                .finish()
                .ok()
                .map(|(_, card)| card);
            // println!("{:?}", x);
            x
        })
        .collect();
    let total: u32 = cards
        .iter()
        .map(|card| {
            let num_matches = card.as_ref().unwrap().winning_numbers().len() as u32;
            // println!("{:?}", card.as_ref().unwrap().winning_numbers());
            if num_matches == 0 {
                0
            } else {
                2_u32.pow(num_matches - 1)
            }
        })
        .sum();
    println!("{total:?}")
}
