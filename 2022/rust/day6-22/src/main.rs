/// There are a few ways we could tackle this problem:
/// * Set up four variables that are each updated every time we process a character
/// * Set up four variables that each represent a character. When a new character is processed,
///     set the variable with the oldest value to the new value. That we we aren't constantly
///     reassigning new values
/// * Same as first two but use some kind of collection
///     * A struct may be nice here
/// * Set up a string/slice variable that gets updated with each iteration
/// * Same as the second idea, but use two pointer variables (newest, oldest) to keep track of the front and back
use itertools::Itertools;

enum DataType {
    Package,
    Message,
}

fn get_start_of_packet_index(signal: &str, data_type: DataType) -> usize {
    let unique_length = match data_type {
        DataType::Package => 4,
        DataType::Message => 14,
    };
    signal
        // We want to use `windows`, but &str doesn't implement that method. A byte slice is more generic than
        // a string slice, so converting the string into a byte slice allows `windows` to work
        .as_bytes()
        // `windows` basically creates an iterator over a bunch of slices of the given size. So, if I had a string
        // "Hello world" and used `windows(4)`, I would get an iterator for the slices "Hell", "ello", "llo " and so on
        .windows(unique_length)
        // Return the first index where the closure returns true
        .position(|window| window.iter().unique().count() == unique_length)
        // According to the problem we want the index at the END of the marker, not the beginning
        .map(|pos| pos + unique_length)
        .unwrap()
}

fn main() {
    // Input is a single line of text, so we need to pull that line out
    let signal = include_str!("../../../inputs/6-tuning-trouble.txt")
        .lines()
        .next()
        .unwrap();
    let signal_start_index = get_start_of_packet_index(signal, DataType::Message);
    println!("Signal start: {signal_start_index:?}");
}
