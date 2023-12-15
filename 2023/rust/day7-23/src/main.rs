use core::fmt;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let mut hands = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect_vec();

    for hand in hands.clone() {
        println!("{}", hand)
    }
    println!("");
    hands.sort();
    let mut winnings = 0;
    for (i, hand) in hands.clone().iter().enumerate() {
        winnings += (i + 1) as u32 * hand.bid;
        println!("{}, winnings: {}", hand, (i + 1) as u32 * hand.bid)
    }
    println!("Winnings: {}", winnings);
}

fn get_hand_type(cards: &[Card], use_jokers: Option<bool>) -> HandType {
    let num_same_cards = cards
        .iter()
        .sorted()
        .group_by(|&card| card)
        .into_iter()
        .map(|(_, group)| group.count())
        .sorted()
        .rev()
        .collect_vec();
    let mut hand_type = match num_same_cards[..] {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("Found a type we're scared of: {:?}", num_same_cards),
    };
    if use_jokers.is_some() && cards.contains(&Card::Joker) {
        println!("x:{:?}", num_same_cards);
        hand_type = match hand_type {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FiveOfAKind,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::TwoPair => {
                if cards.iter().filter(|card| card == &&Card::Joker).count() == 2 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::HighCard => HandType::OnePair,
        }
    }
    hand_type
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug)]
struct HandParseError;

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_hand, s_bid) = s.split_whitespace().collect_tuple().ok_or(HandParseError)?;
        let cards = s_hand
            .chars()
            .map(|c| Card::try_from(c).map_err(|_| HandParseError))
            .collect::<Result<Vec<Card>, HandParseError>>()?;
        let bid = s_bid.parse::<u32>().map_err(|_| HandParseError)?;
        Ok(Hand {
            hand_type: get_hand_type(&cards, Some(true)),
            cards,
            bid,
        })
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.cards, self.hand_type, self.bid)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HandType::FiveOfAKind => "Five of a kind",
            HandType::FourOfAKind => "Four of a kind",
            HandType::FullHouse => "Full house",
            HandType::ThreeOfAKind => "Three of a kind",
            HandType::TwoPair => "Two pair",
            HandType::OnePair => "One pair",
            HandType::HighCard => "High card",
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Joker,
    Num(u8),
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Joker),
            'T' => Ok(Card::Num(10)),
            c => Ok(Card::Num(c.to_digit(10).unwrap() as u8)),
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Joker => 'J',
            Card::Num(10) => 'T',
            Card::Num(n) => char::from_digit(*n as u32, 10).unwrap(),
        };
        write!(f, "{}", s)
    }
}
