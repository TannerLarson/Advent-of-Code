mod part1;
mod part2;
use part1::part_1;
use part2::part_2;

fn main() {
    let mut sum: u32 = part_1(include_str!("input.txt"));
    println!("{sum:?}");
    sum = part_2(include_str!("input.txt"));
    println!("{sum:?}");
}
