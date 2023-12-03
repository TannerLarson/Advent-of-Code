mod pair;
use pair::Pair;
#[allow(dead_code)]

fn main() {
    let points = b();
    println!("{points:?}");
}

fn a() -> i32 {
    include_str!("../../../inputs/4-camp-cleanup.txt")
        .lines()
        .map(|line| {
            let pair = line.parse::<Pair>().unwrap();
            let redundant = pair.has_redundancy();
            println!("pair: {pair:?}\nredundant: {redundant:?}\n");
            match pair.has_redundancy() {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn b() -> i32 {
    include_str!("../../../inputs/4-camp-cleanup.txt")
        .lines()
        .map(|line| {
            let pair = line.parse::<Pair>().unwrap();
            let redundant = pair.has_overlap();
            println!("pair: {pair:?}\noverlap: {redundant:?}\n");
            match pair.has_overlap() {
                true => 1,
                false => 0,
            }
        })
        .sum()
}
