fn main() {
    let input: Vec<String> = include_str!("input.txt")
        .split(',')
        .map(|s| s.to_string().replace('\n', ""))
        .collect();

    let part_1: u32 = input.iter().map(|s| hash(s) as u32).sum();
    println!("Part 1: {}", part_1)
}

fn hash(string: &str) -> u8 {
    let mut current_value = 0_u32;
    for c in string.chars() {
        current_value += (c as u8) as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}
