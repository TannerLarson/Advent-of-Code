use core::fmt;
use grid::Grid;

// This module extends the functionality of the grid crate with some functions I often use when working with grids

pub trait GridExt<T> {
    fn print(&self);
    fn from_str(string: &str) -> Grid<T>;
}

// The `where` clause forces any T to implement the defined traits
impl<T> GridExt<T> for Grid<T>
where
    T: fmt::Debug,
    T: From<char>,
{
    fn print(&self) {
        for row in self.iter_rows() {
            for item in row {
                print!("{:?} ", item)
            }
            println!()
        }
        println!()
    }

    fn from_str(string: &str) -> Grid<T> {
        Grid::from_vec(
            string
                .lines()
                .flat_map(|line| line.chars().map(T::from))
                .collect(),
            string.lines().next().unwrap().len(),
        )
    }
}
