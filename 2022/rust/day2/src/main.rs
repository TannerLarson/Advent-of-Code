use std::str::FromStr;
#[allow(dead_code)]
/// Use in both challenges ////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let points = b();
    println!("{points:?}");
}

/// Move ///
#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn points(&mut self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveError;

impl TryFrom<char> for Move {
    type Error = ParseMoveError;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => panic!("Proveded character \'{letter}\' is not valid!"),
        }
    }
}

/// Outcome ///
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
struct ParseOutcomeError;

impl TryFrom<char> for Outcome {
    type Error = ParseOutcomeError;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => panic!("Proveded character \'{letter}\' is not valid!"),
        }
    }
}

impl Outcome {
    fn points(&mut self) -> u64 {
        match self {
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::Win => 6,
        }
    }
}

/// Challenge A /////////////////////////////////////////////////////////////////////////////////////////////////////

fn a() -> u64 {
    include_str!("../../../inputs/2-rock-paper-scissors.txt")
        .lines()
        .map(TwoMoveRound::from_str)
        .map(|round| round.unwrap().points())
        .sum()
}

/// TwoMoveRound ///
struct TwoMoveRound {
    our_move: Move,
    their_move: Move,
}

impl TwoMoveRound {
    fn outcome(&mut self) -> Outcome {
        if self.our_move == self.their_move {
            return Outcome::Draw;
        }
        let we_win = match self.our_move {
            Move::Rock => self.their_move == Move::Scissors,
            Move::Paper => self.their_move == Move::Rock,
            Move::Scissors => self.their_move == Move::Paper,
        };
        if we_win {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn points(&mut self) -> u64 {
        self.outcome().points() + self.our_move.points()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRoundError;

impl FromStr for TwoMoveRound {
    type Err = ParseRoundError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let their_letter = chars.next().unwrap();
        let our_letter = chars.nth(1).unwrap();

        Ok(TwoMoveRound {
            our_move: Move::try_from(our_letter).unwrap(),
            their_move: Move::try_from(their_letter).unwrap(),
        })
    }
}

/// Challenge B /////////////////////////////////////////////////////////////////////////////////////////////////////
fn b() -> u64 {
    include_str!("../../../inputs/2-rock-paper-scissors.txt")
        .lines()
        .map(OneMoveRound::from_str)
        .map(|round| round.unwrap().points())
        .sum()
}

/// OneMoveRound
struct OneMoveRound {
    their_move: Move,
    outcome: Outcome,
}

impl OneMoveRound {
    fn calculate_your_move(&mut self) -> Move {
        match self.outcome {
            Outcome::Win => match self.their_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Lose => match self.their_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Draw => self.their_move,
        }
    }

    fn points(&mut self) -> u64 {
        self.outcome.points() + self.calculate_your_move().points()
    }
}

impl FromStr for OneMoveRound {
    type Err = ParseRoundError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let their_letter = chars.next().unwrap();
        let outcome_letter = chars.nth(1).unwrap();

        Ok(OneMoveRound {
            their_move: Move::try_from(their_letter).unwrap(),
            outcome: Outcome::try_from(outcome_letter).unwrap(),
        })
    }
}
