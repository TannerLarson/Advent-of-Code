use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1},
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::preceded,
    Finish, IResult,
};

fn word_to_number(i: &str) -> IResult<&str, Option<u32>> {
    alt((
        map(tag("twone"), |_| Some(21)),
        map(tag("oneight"), |_| Some(18)),
        map(tag("threeight"), |_| Some(38)),
        map(tag("fiveight"), |_| Some(58)),
        map(tag("nineight"), |_| Some(98)),
        map(tag("eightwo"), |_| Some(82)),
        map(tag("eighthree"), |_| Some(83)),
        map(tag("sevenine"), |_| Some(79)),
        map(tag("one"), |_| Some(1)),
        map(tag("two"), |_| Some(2)),
        map(tag("three"), |_| Some(3)),
        map(tag("four"), |_| Some(4)),
        map(tag("five"), |_| Some(5)),
        map(tag("six"), |_| Some(6)),
        map(tag("seven"), |_| Some(7)),
        map(tag("eight"), |_| Some(8)),
        map(tag("nine"), |_| Some(9)),
        map(anychar, |_| None),
    ))(i)
}

fn extract_numbers(i: &str) -> Vec<u32> {
    if i.chars().next().unwrap().is_alphabetic() {
        let mut numbers = many0(word_to_number)(i).unwrap().1;
        numbers.retain(Option::is_some);
        numbers
            .into_iter()
            .flat_map(|x| {
                let num = x.unwrap();
                if num > 10 {
                    vec![num / 10, num % 10]
                } else {
                    vec![num]
                }
            })
            .collect()
    } else {
        i.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    }
}

fn parse_letters_or_numbers(i: &str, is_alpha: bool) -> IResult<&str, &str> {
    if is_alpha {
        alpha1(i)
    } else {
        digit1(i)
    }
}

fn parse_line(i: &str) -> IResult<&str, Vec<&str>> {
    let mut is_alpha = i.chars().next().unwrap().is_alphabetic();
    let (mut i, c) = parse_letters_or_numbers(i, is_alpha)?;
    let mut v = vec![c];

    // Consume the input until everything has been parsed
    loop {
        is_alpha = !is_alpha;
        let parser = |i| parse_letters_or_numbers(i, is_alpha);
        let (next_i, maybe_c) = opt(parser)(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i
    }

    Ok((i, v))
}

pub fn part_2(input: &str) -> u32 {
    let mut lines = input.lines();

    (&mut lines)
        .map_while(|line| {
            all_consuming(parse_line)(line).finish().ok().map(|(_, v)| {
                // println!("{v:?}");
                v.iter()
                    .flat_map(|chunk| extract_numbers(chunk))
                    .collect_vec()
            })
        })
        .map(|line| {
            // println!("{line:?}");
            let combined = match line.len() == 1 {
                true => line.first().unwrap() * 11,
                false => line.first().unwrap() * 10 + line.last().unwrap(),
            };
            println!("{combined:?}");
            combined
        })
        .sum()
}
