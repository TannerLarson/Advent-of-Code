use core::fmt;
use grid::Grid;
use lib_rust::grid::{Coord, Direction, GridExt};
use std::collections::{HashMap, HashSet};

fn main() {
    let input_grid = Grid::from_str(include_str!("ex1.txt"));
    let (rows, cols) = (input_grid.rows(), input_grid.cols());
    let mut contraption = Contraption(input_grid, Grid::new(rows, cols));
    println!("Part 1: {}", contraption.get_num_energized())
}

struct Loop(HashSet<(Coord, Direction)>);

struct Contraption(Grid<Part>, Grid<u32>);

impl Contraption {
    fn get_max_energized(grid: &Grid<Part>) -> usize {
        // Use the `Loop` struct to record all coordinates and directions for each part of the loop
        // How do I build the loop?
        // When I hit the `continue` part of `get_num_energized`, I know I've reached a loop
        // I could associate a numerical value with each coord/direction pair that equals the number energized at that point
        4
    }

    fn get_num_energized(&mut self) -> usize {
        // TODO: I need to be able to calculate the current coordinate value based on previously visited coordinates
        // The current coordinate value equals the sum of the values the current coordinate points to plus 1
        let mut engergized_coords: HashMap<Coord, HashSet<Direction>> = HashMap::new();
        let mut stack = vec![(Coord::new(0, 0), Direction::East)];
        while let Some((coord, dir)) = stack.pop() {
            let dirs = engergized_coords.entry(coord).or_default();
            if dirs.contains(&dir) {
                // We have found a loop
                continue;
            } else {
                // Add a new direction at the current coordinate
                dirs.insert(dir);
            }

            for (coord, dir) in self.get_next_coords(coord, dir) {
                if let Some(coord) = coord {
                    // Add next coordinates to the stack
                    stack.push((coord, dir))
                } else {
                    // We've reached a dead end
                }
            }
        }
        engergized_coords.len()
    }

    fn get_next_coords(&self, coord: Coord, dir: Direction) -> Vec<(Option<Coord>, Direction)> {
        match self.0.get_at_coord(&coord).unwrap() {
            Part::FMirror => {
                let new_dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                vec![(self.0.relative_coord(&coord, &new_dir), new_dir)]
            }
            Part::BMirror => {
                let new_dir = match dir {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                };
                vec![(self.0.relative_coord(&coord, &new_dir), new_dir)]
            }
            Part::HSplitter => {
                let mut coords = Vec::new();
                if let Direction::North | Direction::South = dir {
                    coords.push((
                        self.0.relative_coord(&coord, &Direction::West),
                        Direction::West,
                    ));
                    coords.push((
                        self.0.relative_coord(&coord, &Direction::East),
                        Direction::East,
                    ));
                } else {
                    coords.push((self.0.relative_coord(&coord, &dir), dir))
                }
                coords
            }
            Part::VSplitter => {
                let mut coords = Vec::new();
                if let Direction::East | Direction::West = dir {
                    coords.push((
                        self.0.relative_coord(&coord, &Direction::North),
                        Direction::North,
                    ));
                    coords.push((
                        self.0.relative_coord(&coord, &Direction::South),
                        Direction::South,
                    ));
                } else {
                    coords.push((self.0.relative_coord(&coord, &dir), dir))
                }
                coords
            }
            Part::Empty => {
                vec![(self.0.relative_coord(&coord, &dir), dir)]
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Part {
    Empty,
    FMirror,
    BMirror,
    VSplitter,
    HSplitter,
}

impl From<char> for Part {
    fn from(value: char) -> Self {
        match value {
            '.' => Part::Empty,
            '/' => Part::FMirror,
            '\\' => Part::BMirror,
            '|' => Part::VSplitter,
            '-' => Part::HSplitter,
            _ => panic!("At the disco: {}", value),
        }
    }
}

impl From<Part> for char {
    fn from(value: Part) -> Self {
        match value {
            Part::Empty => '.',
            Part::FMirror => '/',
            Part::BMirror => '\\',
            Part::VSplitter => '|',
            Part::HSplitter => '-',
        }
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
