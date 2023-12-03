mod parse_crates {
    use core::fmt;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::{map, opt},
        sequence::{delimited, preceded},
        IResult,
    };

    #[derive(Clone, Copy)]
    pub struct Crate(char);

    impl fmt::Debug for Crate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Display for Crate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    // IResult is a special kind of return type: `Result<(I, O), Err<E>>`. I is the string left after parsing,
    // O is the string the parsing extracted.
    fn parse_crate(i: &str) -> IResult<&str, Crate> {
        // Function that gets the first char from the string
        let first_char = |s: &str| Crate(s.chars().next().unwrap());
        // Parser that takes the single value in between '[' and ']'
        let f = delimited(tag("["), take(1_usize), tag("]"));
        // 1. Use the parser on i   2. Run first_char on the parser result
        // It's basically getting a char in between two square brackets
        map(f, first_char)(i)
    }

    fn parse_hole(i: &str) -> IResult<&str, ()> {
        // drop is a special keyword that disposes of the value given it
        // So, this gets a triple empty space then drops it from the string. Basically it just strips the string of a " "
        map(tag("   "), drop)(i)
    }

    fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
        // alt just runs the given parsers until one succeeds
        // First it tries to parse a crate then extract the value using `Some`
        // Then it tries to parse a hole. If this succeeds it will return None as the O value in IResult
        // We need to use map on `parse_hole` because we want to represent holes as None
        alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
    }

    pub fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
        // Parse out our first crate or hole
        let (mut i, c) = parse_crate_or_hole(i)?;
        // Create a vector to contain the crates on each horizontal line
        let mut v = vec![c];

        loop {
            // Try to parse out another crate or hole that is preceded by a " "
            // This makes sense because if we have "  " then it will treat it as a single hole
            // Note here that `next_i` is the value we get after parsing i. We are essentially picking off
            //   parts of i until no "  " or "[x]" are left
            let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
            // If we got a crate or a hole, push it onto v. Otherwise, we are at the end of the line and should stop parsing
            match maybe_c {
                Some(c) => v.push(c),
                None => break,
            }
            // set i to the value left over after parsing
            i = next_i;
        }

        Ok((i, v))
    }

    pub fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
        // Make sure our top-level vec isn't empty
        assert!(!v.is_empty());
        let len = v[0].len();
        // Make a vector that contains iterators for the bottom-level vectors
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        // This is the transposing part
        // Breaking it down:
        // 1. Get a range (In our case 0 - 8)
        // 2. For each number, loop through each iterator of the bottom-level vectors
        // 3. Call `next().unwrap()` on each iterator
        // 4. Collect the resulting values into a vector
        // 5. Repeat 2-4 until we our through our range and we have a list of full vectors
        // 6. Collect our list of vectors into a bigger vector
        // 7. We now have a transposed vector of vectors!
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .rev()
                    .filter_map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

mod parse_instructions {
    use nom::{
        bytes::complete::{tag, take_while1},
        combinator::{map, map_res},
        sequence::{preceded, tuple},
        IResult,
    };

    fn parse_number(i: &str) -> IResult<&str, usize> {
        // `map_res` is basically `map` but instead of expecting f to return a value it expects f to return a Result
        // `take_while1` parses the string and returns the longest slice that follows the given condition
        // So basically we are parsing out the first longest value that is an ascii digit.
        map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
            s.parse::<usize>()
        })(i)
    }

    fn parse_pile_number(i: &str) -> IResult<&str, usize> {
        // Just call `parse_number` but return 1 - the return value
        // We're doing this because the text file goes from 1 - 9 and our crate_columns vector goes from 0 - 8
        map(parse_number, |i| i - 1)(i)
    }

    #[derive(Debug)]
    pub struct Instruction {
        pub quantity: usize,
        pub src: usize,
        pub dst: usize,
    }

    pub fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
        map(
            // `tuple` runs several parsers and combines the result of each parse into a tuple that it returns
            tuple((
                preceded(tag("move "), parse_number),
                preceded(tag(" from "), parse_pile_number),
                preceded(tag(" to "), parse_pile_number),
            )),
            // Build an Instruction based on the parsed line
            |(quantity, src, dst)| Instruction { quantity, src, dst },
        )(i)
    }
}

use core::fmt;
use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use parse_crates::{parse_crate_line, transpose_rev, Crate};
use parse_instructions::{parse_instruction, Instruction};

struct Piles(Vec<Vec<Crate>>);

impl Piles {
    fn apply_single(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            // Remove element from top of one crate
            let el = self.0[ins.src].pop().unwrap();
            // Put the element on top of another
            self.0[ins.dst].push(el);
        }
    }

    fn apply_multiple(&mut self, ins: Instruction) {
        for krate in (0..ins.quantity)
            .map(|_| self.0[ins.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.dst].push(krate);
        }
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

fn main() {
    let mut lines = include_str!("../../../inputs/5-supply-stacks.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        // Same as the `map` function but skips mapping for any value that doesn't return Some
        .map_while(|line| {
            // Runs the parsing method until there isn't anything left to parse
            all_consuming(parse_crate_line)(line)
                // This line isn't necessary, just help with error handling
                .finish()
                .ok()
                // We only care about what was parsed out of the string. We can discard the leftover string.
                .map(|(_, line)| line)
        })
        .collect();
    // Turn our vectors of lines into vectors that acutally represent a crate pile
    let mut piles = Piles(transpose_rev(crate_lines));
    println!("{piles:?}");

    // Take care of the line between the numbers line and instructions
    assert!(lines.next().unwrap().is_empty());

    // Parse the rest of the lines in the file
    // We only care about the values parsed out, hence the `.1`
    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply_multiple(ins);
        println!("{piles:?}");
    }

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}
