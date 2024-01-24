use core::fmt;
use grid::Grid;

// This module extends the functionality of the grid crate with some functions I often use when working with grids

pub trait GridExt<T> {
    fn print(&self);
    fn from_str(string: &str) -> Grid<T>;
    fn get_at_coord(&self, coord: &Coord) -> Option<&T>;
    fn get_at_coord_mut(&mut self, coord: &Coord) -> Option<&mut T>;
    fn relative_coord(&self, coord: &Coord, dir: &Direction) -> Option<Coord>;
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

    fn get_at_coord(&self, coord: &Coord) -> Option<&T> {
        self.get(coord.row, coord.col)
    }

    fn get_at_coord_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        self.get_mut(coord.row, coord.col)
    }

    fn relative_coord(&self, coord: &Coord, dir: &Direction) -> Option<Coord> {
        let (coord_row, coord_col) = (coord.row as i32, coord.col as i32);
        let (row, col) = match dir {
            Direction::North => (coord_row - 1, coord_col),
            Direction::East => (coord_row, coord_col + 1),
            Direction::South => (coord_row + 1, coord_col),
            Direction::West => (coord_row, coord_col - 1),
        };
        if row < 0 || col < 0 || row as usize >= self.rows() || col as usize >= self.cols() {
            None
        } else {
            Some(Coord {
                row: row as usize,
                col: col as usize,
            })
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
