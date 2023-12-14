use core::fmt;
use roots::{find_roots_quadratic, Roots};
use std::ops::Range;

fn main() {
    let mut parsed_lines = include_str!("input.txt").lines().map(|s| {
        s.split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<u64>>()
    });
    let times = parsed_lines.next().unwrap();
    let distances = parsed_lines.next().unwrap();

    println!("Part 1: {}", get_answer(times, distances));
    // let times = vec![71530];
    // let distances = vec![940200];
    let times = vec![49787980];
    let distances = vec![298118510661181];
    println!("Part 2: {}", get_answer(times, distances))
}

fn get_answer(times: Vec<u64>, distances: Vec<u64>) -> u64 {
    let races: Vec<Race> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    races
        .iter()
        .map(|race| {
            let times = race.better_button_press_times();
            println!("{:?}: {:?}", times, (times.end - times.start));
            times.end - times.start
        })
        .product()
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn better_button_press_times(&self) -> Range<u64> {
        let (time, distance) = (self.time as f64, self.distance as f64);
        match find_roots_quadratic(1f64, time * -1f64, distance) {
            Roots::Two(roots) => {
                let low = if roots[0].fract() == 0.0 {
                    (roots[0] as u64) + 1
                } else {
                    roots[0].ceil() as u64
                };
                let high = if roots[1].fract() == 0.0 {
                    (roots[1] as u64) - 1
                } else {
                    roots[1].floor() as u64
                };
                low..(high + 1)
            }
            _ => panic!("Quadratic equation failed"),
        }
    }
}

impl fmt::Debug for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t:{:?}, d:{:?}", self.time, self.distance)
    }
}
