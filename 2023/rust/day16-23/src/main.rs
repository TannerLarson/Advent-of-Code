use core::fmt;
use grid::Grid;
use lib_rust::grid::{Coord, Direction, GridDisplay, GridExt};
use std::collections::{HashMap, HashSet};

fn main() {
    let input_grid = Grid::from_str(include_str!("ex1.txt"));
    let (rows, cols) = (input_grid.rows(), input_grid.cols());
    let mut contraption = Contraption {
        grid: input_grid,
        energized: Grid::new(rows, cols),
    };
    println!(
        "Part 1: {}",
        contraption.get_num_energized(Coord::new(0, 0), Direction::East)
    );
    println!("Part 2: {}", contraption.get_max_energized());
}

struct Contraption {
    grid: Grid<Part>,
    energized: Grid<HashMap<Direction, HashSet<Coord>>>,
}

impl Contraption {
    fn get_max_energized(&mut self) -> u32 {
        // Use the `Loop` struct to record all coordinates and directions for each part of the loop
        // How do I build the loop?
        // When I hit the `continue` part of `get_num_energized`, I know I've reached a loop
        // I could associate a numerical value with each coord/direction pair that equals the number energized at that point

        self.get_num_energized(Coord::new(2, 0), Direction::East);

        // let west = (0..(self.grid.rows()))
        //     .map(|i| self.get_num_energized(Coord::new(i, 0), Direction::East))
        //     .max()
        //     .unwrap();
        // println!("West: {}", west);
        // let east = (0..(self.grid.rows()))
        //     .map(|i| self.get_num_energized(Coord::new(i, self.grid.cols() - 1), Direction::West))
        //     .max()
        //     .unwrap();
        // println!("East: {}", east);
        // let north = (0..(self.grid.cols()))
        //     .map(|i| self.get_num_energized(Coord::new(0, i), Direction::South))
        //     .max()
        //     .unwrap();
        // println!("North: {}", north);
        // let south = (0..(self.grid.cols()))
        //     .map(|i| self.get_num_energized(Coord::new(self.grid.rows() - 1, i), Direction::North))
        //     .max()
        //     .unwrap();
        // println!("South: {}", south);
        // *[west, east, north, south].iter().max().unwrap()
        4
    }

    // If we have already visited the coord in the defined direction and the value at that coord and direction is zero
    fn already_visited(&self, coord: &Coord, dir: &Direction) -> bool {
        self.energized.get_at_coord(coord).is_some()
            && self
                .energized
                .get_at_coord(coord)
                .unwrap()
                .contains_key(dir)
    }

    // If we can calculate the current coords child coords, do so. Otherwise, return None
    fn collect_child_coords(
        &self,
        coord: Coord,
        next_coords: &[(Option<Coord>, Direction)],
    ) -> Option<HashSet<Coord>> {
        if !next_coords.iter().all(|(next_coord, next_dir)| {
            next_coord.is_none() || self.already_visited(&next_coord.unwrap(), next_dir)
        }) {
            return None;
        }

        let mut combined_set: HashSet<Coord> = next_coords
            .iter()
            .filter(|(coord, _)| coord.is_some())
            .flat_map(|(next_coord, next_dir)| {
                self.energized
                    .get_at_coord(&next_coord.unwrap())
                    .unwrap()
                    .get(next_dir)
                    .unwrap()
                    .clone()
            })
            .collect();

        combined_set.insert(coord);
        Some(combined_set)
    }

    fn get_num_energized(&mut self, start_coord: Coord, start_dir: Direction) -> u32 {
        // TODO: I need to be able to calculate the current coordinate value based on previously visited coordinates
        // The current coordinate value equals the sum of the values the current coordinate points to plus 1
        // We need to record the value for each coordinate in each direction
        println!("{:?} {:?}", start_coord, start_dir);
        let mut in_loop = false;
        let mut stack = vec![(start_coord, start_dir)];
        while let Some((coord, dir)) = stack.pop() {
            println!("{:?}", coord);
            // Check if we can calculate the current coordinate's value
            let next_coords = self.get_next_coords(coord, dir);
            let set = if let Some(collected_set) = self.collect_child_coords(coord, &next_coords) {
                println!("Calculated: {}", collected_set.len());
                collected_set
            } else {
                stack.push((coord, dir));
                for (next_coord, next_dir) in next_coords {
                    if let Some(next_coord) = next_coord {
                        if !self.already_visited(&next_coord, &next_dir) {
                            stack.push((next_coord, next_dir))
                        }
                    }
                }
                HashSet::new()
            };

            let dirs = self.energized.get_at_coord_mut(&coord).unwrap();
            if ([Direction::East, Direction::West].contains(&dir))
                && *self.grid.get_at_coord(&coord).unwrap() == Part::VSplitter
            {
                dirs.insert(Direction::East, set.clone());
                dirs.insert(Direction::West, set);
            } else if ([Direction::North, Direction::South].contains(&dir))
                && *self.grid.get_at_coord(&coord).unwrap() == Part::HSplitter
            {
                dirs.insert(Direction::North, set.clone());
                dirs.insert(Direction::South, set);
            } else {
                dirs.insert(dir, set);
            }
        }
        dbg!(self
            .energized
            .get_at_coord(&start_coord)
            .unwrap()
            .get(&start_dir)
            .unwrap()
            .len() as u32)
    }

    fn get_next_coords(&self, coord: Coord, dir: Direction) -> Vec<(Option<Coord>, Direction)> {
        match self.grid.get_at_coord(&coord).unwrap() {
            Part::FMirror => {
                let new_dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                vec![(self.grid.relative_coord(&coord, &new_dir), new_dir)]
            }
            Part::BMirror => {
                let new_dir = match dir {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                };
                vec![(self.grid.relative_coord(&coord, &new_dir), new_dir)]
            }
            Part::HSplitter => {
                let mut coords = Vec::new();
                if let Direction::North | Direction::South = dir {
                    coords.push((
                        self.grid.relative_coord(&coord, &Direction::West),
                        Direction::West,
                    ));
                    coords.push((
                        self.grid.relative_coord(&coord, &Direction::East),
                        Direction::East,
                    ));
                } else {
                    coords.push((self.grid.relative_coord(&coord, &dir), dir))
                }
                coords
            }
            Part::VSplitter => {
                let mut coords = Vec::new();
                if let Direction::East | Direction::West = dir {
                    coords.push((
                        self.grid.relative_coord(&coord, &Direction::North),
                        Direction::North,
                    ));
                    coords.push((
                        self.grid.relative_coord(&coord, &Direction::South),
                        Direction::South,
                    ));
                } else {
                    coords.push((self.grid.relative_coord(&coord, &dir), dir))
                }
                coords
            }
            Part::Empty => {
                vec![(self.grid.relative_coord(&coord, &dir), dir)]
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
