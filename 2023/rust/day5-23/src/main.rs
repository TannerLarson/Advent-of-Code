fn main() {
    let lines = include_str!("./ex1.txt").lines();
    lines.for_each(|line| println!("{}", line))
}
