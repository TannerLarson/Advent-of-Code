mod item;
use im::HashSet as IMHashSet;
use item::Item;
use itertools::Itertools;
use std::collections::HashSet;

#[allow(dead_code)]

fn main() {
    let points = b();
    println!("{points:?}");
}

fn a() -> color_eyre::Result<()> {
    // Read in line
    // Split line into two compartments
    // Find the one similarity between the two compartments
    // Convert that similarity into a score
    // Add the score to the total
    let sum = include_str!("../../../inputs/3-rucksack-reorganization.txt")
        .lines()
        .map(|line| -> color_eyre::Result<_> {
            let (left, right) = line.split_at(line.len() / 2);
            let left_items = left
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;
            itertools::process_results(right.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| left_items.contains(&item))
                    .map(|item| dbg!(item.priority()))
                    .ok_or_else(|| color_eyre::eyre::eyre!("compartments have no items in common"))
            })?
        })
        .sum::<color_eyre::Result<usize>>()?;
    dbg!(sum);
    Ok(())
}

fn b() -> color_eyre::Result<()> {
    let sum: usize = include_str!("../../../inputs/3-rucksack-reorganization.txt")
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| b.try_into().unwrap())
                .collect::<IMHashSet<Item>>()
        })
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            chunks
                .reduce(|a, b| a.intersection(b))
                .expect("we always have 3 chunks")
                .iter()
                .next()
                .expect("Problem statement says there is always one item in common")
                .priority()
        })
        .sum();
    dbg!(sum);
    Ok(())
}
