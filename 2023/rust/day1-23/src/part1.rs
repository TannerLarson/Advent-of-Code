pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter(|c| c.is_ascii_digit());
            let (first, last) = (
                numbers.next().unwrap().to_digit(10).unwrap(),
                numbers.last(),
            );
            match last {
                None => 11 * first,
                Some(c) => 10 * first + c.to_digit(10).unwrap(),
            }
        })
        .sum()
}
