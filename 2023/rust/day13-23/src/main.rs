use core::fmt;

use grid::Grid;
use itertools::Itertools;

fn main() {
    let input = include_str!("ex1.txt");
    let grids = input.split("\n\n").map(parse_grid).collect_vec();
    print_grid(grids.first().unwrap())
}

#[derive(Default)]
enum Object {
    #[default]
    Ash,
    Rock,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Object::Ash,
            '#' => Object::Rock,
            _ => panic!("Bad char: {}", value),
        }
    }
}

impl From<&Object> for char {
    fn from(value: &Object) -> Self {
        match value {
            Object::Ash => '.',
            Object::Rock => '#',
        }
    }
}

fn parse_grid(input: &str) -> Grid<Object> {
    let num_cols = input.lines().next().unwrap().len();
    let rows = input
        .lines()
        .flat_map(|line| line.chars().map(Object::from))
        .collect_vec();

    Grid::from_vec(rows, num_cols)
}

fn print_grid(grid: &Grid<Object>) {
    for row in grid.iter_rows() {
        for object in row {
            print!("{:?} ", object)
        }
        println!()
    }
}
