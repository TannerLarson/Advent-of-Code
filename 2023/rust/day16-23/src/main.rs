use core::fmt;
use grid::Grid;
use lib_rust::grid::{Coord, Direction, GridDisplay, GridExt};
use std::collections::{HashMap, HashSet};

fn main() {
    let input_grid = Grid::from_str(include_str!("ex1.txt"));
    let (rows, cols) = (input_grid.rows(), input_grid.cols());
    let mut contraption = Contraption(input_grid, Grid::new(rows, cols));
    println!("Part 1: {}", contraption.get_num_energized());
    contraption.0.print();
}

struct Contraption(Grid<Part>, Grid<HashMap<Direction, HashSet<Coord>>>);

impl Contraption {
    fn get_max_energized(grid: &Grid<Part>) -> usize {
        // Use the `Loop` struct to record all coordinates and directions for each part of the loop
        // How do I build the loop?
        // When I hit the `continue` part of `get_num_energized`, I know I've reached a loop
        // I could associate a numerical value with each coord/direction pair that equals the number energized at that point
        4
    }

    // If next values are either None or we have already visited them at the proper direction
    fn can_calculate_next_value(&self, next_coords: &[(Option<Coord>, Direction)]) -> bool {
        next_coords.iter().all(|(next_coord, next_dir)| {
            next_coord.is_none()
                || self
                    .1
                    .get_at_coord(&next_coord.unwrap())
                    .unwrap()
                    .contains_key(next_dir)
        })
    }

    // If we have already visited the coord in the defined direction and the value at that coord and direction is zero
    fn already_visited(&self, coord: &Coord, dir: &Direction) -> bool {
        let set = self.1.get_at_coord(coord);
        set.is_some() && self.1.get_at_coord(coord).unwrap().contains_key(dir)
    }

    fn set_value_at_coord(
        &mut self,
        coord: Coord,
        dir: Direction,
        next_coords: &[(Option<Coord>, Direction)],
    ) {
        // Iterator over HashSets of next coordinates
        let combined_set: HashSet<Coord> = next_coords
            .iter()
            .filter_map(|(next_coord, next_dir)| {
                if next_coord.is_some() {
                    let map = self.1.get_at_coord(&next_coord.unwrap()).unwrap();
                    if map.contains_key(next_dir) {
                        Some(map.get(&dir).unwrap().clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        let len = combined_set.len();
        *self
            .1
            .get_at_coord_mut(&coord)
            .unwrap()
            .entry(dir)
            .or_default() = combined_set;
        println!("Calculated value: {}", len);
    }

    fn get_num_energized(&mut self) -> u32 {
        // TODO: I need to be able to calculate the current coordinate value based on previously visited coordinates
        // The current coordinate value equals the sum of the values the current coordinate points to plus 1
        // We need to record the value for each coordinate in each direction
        let mut stack = vec![(Coord::new(0, 0), Direction::East)];
        while let Some((coord, dir)) = stack.pop() {
            println!("{:?}", coord);
            // Check if we can calculate the current coordinate's value
            let next_coords = self.get_next_coords(coord, dir);
            if self.can_calculate_next_value(&next_coords) {
                self.set_value_at_coord(coord, dir, &next_coords);
                continue;
            } else {
                println!("Pushing coords");
                // Push current coord
                stack.push((coord, dir));
                // Push next coord(s)
                for (next_coord, next_dir) in next_coords {
                    if let Some(next_coord) = next_coord {
                        if !self.already_visited(&next_coord, &next_dir) {
                            stack.push((next_coord, next_dir))
                        }
                    }
                }
            }

            let dirs = self.1.get_at_coord_mut(&coord).unwrap();
            if dirs.contains_key(&dir) {
                // We have found a loop
                // This is getting in the way of adding. When we find the base case and go back one, it thinks
                // we have found a loop and doesn't add the base case
                println!("Loop found");
            } else {
                // Add a new direction(s) at the current coordinate
                let mut set = HashSet::new();
                set.insert(coord);
                if ([Direction::East, Direction::West].contains(&dir))
                    && *self.0.get_at_coord(&coord).unwrap() == Part::VSplitter
                {
                    dirs.insert(Direction::East, set.clone());
                    dirs.insert(Direction::West, set);
                } else if ([Direction::North, Direction::South].contains(&dir))
                    && *self.0.get_at_coord(&coord).unwrap() == Part::HSplitter
                {
                    dirs.insert(Direction::North, set.clone());
                    dirs.insert(Direction::South, set);
                } else {
                    dirs.insert(dir, set);
                }
            }
        }
        self.1
            .get(0, 0)
            .unwrap()
            .get(&Direction::East)
            .unwrap()
            .len() as u32
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

#[derive(Clone, Copy, PartialEq)]
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
