use core::fmt;
use std::cmp::max;
use std::ops::{Add, AddAssign};

use colored::Colorize;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::{many1, many_m_n},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Copy, Clone)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl Cubes {
    fn new() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

impl AddAssign<Cubes> for Cubes {
    fn add_assign(&mut self, rhs: Cubes) {
        self.red += rhs.red;
        self.blue += rhs.blue;
        self.green += rhs.green;
    }
}

impl fmt::Debug for Cubes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.red.to_string().red(),
            self.green.to_string().green(),
            self.blue.to_string().cyan()
        )
    }
}

#[derive(Clone)]
struct Game {
    id: u32,
    handfuls: Vec<Cubes>,
}

impl Game {
    fn fewest_possible(&self) -> Cubes {
        let mut cubes = Cubes::new();
        for handful in self.handfuls.clone() {
            cubes.blue = max(cubes.blue, handful.blue);
            cubes.green = max(cubes.green, handful.green);
            cubes.red = max(cubes.red, handful.red);
        }
        cubes
    }

    fn is_possible(&self, cubes: Cubes) -> bool {
        let min = self.fewest_possible();
        min.red <= cubes.red && min.green <= cubes.green && min.blue <= cubes.blue
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {:?}", self.id, self.handfuls)
    }
}

fn parse_color(i: &str) -> IResult<&str, Color> {
    alt((
        map(tag("blue"), |_| Color::Blue),
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
    ))(i)
}

fn parse_id(i: &str) -> IResult<&str, u32> {
    map(delimited(tag("Game "), digit1, tag(":")), |s: &str| {
        s.parse::<u32>().unwrap()
    })(i)
}

fn parse_color_group(i: &str) -> IResult<&str, Cubes> {
    map(
        separated_pair(digit1, tag(" "), parse_color),
        |(amount, color)| {
            let amount = amount.parse::<u32>().unwrap();
            match color {
                Color::Blue => Cubes {
                    blue: amount,
                    green: 0,
                    red: 0,
                },
                Color::Red => Cubes {
                    red: amount,
                    green: 0,
                    blue: 0,
                },
                Color::Green => Cubes {
                    green: amount,
                    blue: 0,
                    red: 0,
                },
            }
        },
    )(i)
}

fn parse_handful(i: &str) -> IResult<&str, Cubes> {
    map(
        many_m_n(
            1,
            3,
            preceded(alt((tag(" "), tag(", "))), parse_color_group),
        ),
        |color_groups| {
            let mut combined = Cubes::new();
            for cubes in color_groups {
                combined += cubes;
            }
            combined
        },
    )(i)
}

fn parse_game(i: &str) -> Game {
    let (i, id) = parse_id(i).unwrap();

    let handfuls = many1(map(pair(parse_handful, opt(tag(";"))), |(cubes, _)| cubes))(i)
        .unwrap()
        .1;

    Game { id, handfuls }
}

fn part_one(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(parse_game).collect();
    let min_ids = games
        .iter()
        .filter(|game| {
            game.is_possible(Cubes {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id);
    min_ids.sum()
}

fn part_two(input: &str) -> u32 {
    let games: Vec<Game> = input.lines().map(parse_game).collect();
    let powers = games.iter().map(|game| game.fewest_possible().power());
    powers.sum()
}

fn main() {
    let mut ans = part_one(include_str!("input.txt"));
    println!("{ans:?}");
    let ans = part_two(include_str!("input.txt"));
    println!("{ans:?}");
}
