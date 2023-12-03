mod parsing;

use nom::{combinator::all_consuming, Finish};
use parsing::Instruction;

fn main() {
    let instructions = include_str!("../../../inputs/9-rope-bridge.txt")
        .lines()
        .map(|line| all_consuming(Instruction::parse)(line).finish().unwrap().1);

    for ins in instructions {
        println!("{ins:?}");
    }
}
