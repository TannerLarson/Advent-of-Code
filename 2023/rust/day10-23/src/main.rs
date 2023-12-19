use core::fmt;
use grid::{grid, Grid};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let grid = parse_input(include_str!("ex1.txt"));
    for row in grid.iter_rows() {
        println!("{:?}", row.collect_vec());
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Default)]
struct Segment(Vec<Direction>);

impl Segment {
    fn can_connect(&self, seg: &Segment) -> bool {
        match self.0.len() {
            0 => false,
            4 => true,
            2 => self.0.iter().any(|dir| seg.0.contains(&dir.opposite())),
            _ => panic!("We should never get here"),
        }
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug)]
struct ParseSegmentError;

impl TryFrom<char> for Segment {
    type Error = ParseSegmentError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let directions = match value {
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::East, Direction::West],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            '7' => vec![Direction::South, Direction::West],
            'F' => vec![Direction::East, Direction::South],
            '.' => vec![],
            'S' => vec![
                Direction::East,
                Direction::West,
                Direction::North,
                Direction::South,
            ],
            _ => panic!("Trying to parse bad character: {}", value),
        };
        Ok(Segment(directions))
    }
}

fn parse_input(input: &str) -> Grid<Segment> {
    let mut grid = grid![];
    for line in input.lines() {
        grid.push_row(
            line.chars()
                .map(|c| Segment::try_from(c).unwrap())
                .collect_vec(),
        )
    }
    grid
}

struct Pipe(Vec<Segment>);

// fn find_pipes(grid: Grid<Pipe>) -> Vec<Pipe> {
//     let visited: HashSet<GridCoord> = HashSet::new();
//     for coord in grid.all_coords() {
//         if visited.contains(&coord) {
//             continue;
//         }
//     }

//     5
// }

// fn follow_pipe(coord: &GridCoord, grid: Grid<char>, visited: &HashSet<GridCoord>) }{

// fn find_compatible_pipe(coord: &GridCoord, grid: Grid<char>) -> Option<GridCoord> {
//     let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
//     dirs.map(|(x, y)| {
//         let offset_coord = coord.get_relative_coord(x, y);
//         grid.cell()
//     });

//     89
// }
