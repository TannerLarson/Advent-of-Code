use core::fmt;
use itertools::Itertools;

fn main() {
    let sequences = include_str!("input.txt")
        .lines()
        .map(|line| {
            Sequences::new(
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut ans: i64 = sequences.iter().map(|seq| seq.extrapolate_next()).sum();
    println!("Part 1: {}", ans);

    ans = sequences.iter().map(|seq| seq.extrapolate_prev()).sum();
    println!("Part 2: {}", ans);
}

struct Sequences(Vec<Vec<i64>>);

impl Sequences {
    fn new(mut seq: Vec<i64>) -> Self {
        // Find all the sequences based on the base sequence
        let mut sequences = Vec::new();
        loop {
            sequences.push(seq.clone());
            if Sequences::get_next_sequence(&seq).is_none() {
                return Sequences(sequences);
            }
            seq = Sequences::get_next_sequence(&seq).unwrap();
        }
    }

    fn get_next_sequence(seq: &[i64]) -> Option<Vec<i64>> {
        let next: Vec<i64> = seq.iter().tuple_windows().map(|(a, b)| b - a).collect();
        match next.iter().all(|num| *num == 0_i64) {
            true => None,
            false => Some(next),
        }
    }

    fn extrapolate_next(&self) -> i64 {
        self.0.iter().map(|i| i.last().unwrap()).sum()
    }

    fn extrapolate_prev(&self) -> i64 {
        // This could probably be simplified
        let mut prev = 0_i64;
        for seq in self.0.iter().rev() {
            prev = seq.first().unwrap() - prev;
        }
        prev
    }
}

impl fmt::Debug for Sequences {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
