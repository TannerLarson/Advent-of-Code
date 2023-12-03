use itertools::Itertools;

fn main() {
    let max = b();
    println!("{max:?}");
}

#[allow(dead_code)]
fn a() -> Option<u64> {
    include_str!("../../../inputs/1-calorie-counting.txt")
        .lines()
        .map(|calorie_value| calorie_value.parse::<u64>().ok())
        .batching(|it| {a
            let mut sum = None;
            while let Some(Some(calorie_value)) = it.next() {
                sum = Some(sum.unwrap_or(0) + calorie_value);
            }
            sum
        })
        .max()
}

fn b() -> u64 {
    include_str!("../../../inputs/1-calorie-counting.txt")
        .lines()
        .map(|calorie_value| calorie_value.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(calorie_value)) = it.next() {
                sum = Some(sum.unwrap_or(0) + calorie_value);
            }
            sum
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}
