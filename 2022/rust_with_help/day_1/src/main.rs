// https://fasterthanli.me/series/advent-of-code-2022/part-1

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max = a();
    println!("{max:?}");

    Ok(())
}

fn a() -> Option<u64> {
    include_str!("../../../inputs/1-calorie-counting.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
}
