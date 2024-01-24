use core::fmt;

use grid::Grid;

pub trait GridExt<T> {
    fn print(&self);
}

impl<T: fmt::Debug> GridExt<T> for Grid<T> {
    fn print(&self) {
        for row in self.iter_rows() {
            for item in row {
                print!("{:?} ", item)
            }
            println!()
        }
        println!()
    }
}
