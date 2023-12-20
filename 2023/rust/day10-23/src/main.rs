use core::{fmt, panic};
use grid::{grid, Grid};
use itertools::Itertools;

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    let start = match grid
        .indexed_iter()
        .filter_map(|(coord, seg)| {
            if seg.0.len() == 4 {
                Some(GridCoord::new(coord))
            } else {
                None
            }
        })
        .next()
    {
        Some(coord) => coord,
        None => panic!("Couldn't find start"),
    };

    let pipe = Pipe::build(start, &grid).unwrap();

    println!("{:?}", pipe);
    println!("Part 1: {}", pipe.0.len() / 2)
}

trait GridExt<T> {
    fn get_with_coord(&self, coord: &GridCoord) -> Option<&T>;
}

impl<T> GridExt<T> for Grid<T> {
    fn get_with_coord(&self, coord: &GridCoord) -> Option<&T> {
        self.get(coord.x, coord.y)
    }
}

#[derive(PartialEq, Clone, Copy)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl GridCoord {
    fn new(coord: (usize, usize)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
        }
    }

    fn get_relative_coord(&self, dir: Direction) -> Self {
        match dir {
            Direction::East => GridCoord::new((self.x + 1, self.y)),
            Direction::North => GridCoord::new((self.x, self.y + 1)),
            Direction::South => GridCoord::new((self.x, self.y - 1)),
            Direction::West => GridCoord::new((self.x - 1, self.y)),
        }
    }
}

#[derive(Debug)]
struct DirectionToCoordError;

impl fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

#[derive(Default, Clone)]
struct Segment(Vec<Direction>);

impl Segment {
    fn can_connect(&self, direction: Direction) -> bool {
        // println!(
        //     "Do any of these directions equal {:?}: {:?}",
        //     direction, self.0
        // );
        self.0.iter().any(|dir| *dir == direction)
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

struct Pipe(Vec<(GridCoord, Segment)>);

impl Pipe {
    fn build(coord_start: GridCoord, grid: &Grid<Segment>) -> Option<Self> {
        let mut pipe_coords: Vec<GridCoord> = Vec::new();
        let mut current_coord = coord_start;
        let mut prev_coord: Option<GridCoord> = None;

        loop {
            // println!("\npipe: {:?}", pipe_coords);
            // println!("current: {:?}", current_coord);
            // println!("prev: {:?}", prev_coord);
            let next_coords = adjacent_compatible_coords(&current_coord, grid)
                .iter()
                // filter out the previously visited coordinate
                .filter(|coord| match prev_coord {
                    Some(c) => **coord != c,
                    None => true,
                })
                .cloned()
                .collect_vec();
            pipe_coords.push(current_coord);
            prev_coord = Some(current_coord);
            current_coord = *next_coords.first().unwrap();

            if current_coord == coord_start {
                break;
            }
        }
        let pipe = Pipe(
            pipe_coords
                .into_iter()
                .map(|coord| (coord, grid.get_with_coord(&coord).cloned().unwrap()))
                .collect_vec(),
        );
        Some(pipe)
    }
}

impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pipe [")?;
        for (coord, seg) in &self.0 {
            writeln!(f, "    {:?}: {:?}", coord, seg)?;
        }
        writeln!(f, "]")?;
        Ok(())
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
    grid.rotate_right();
    grid
}

fn adjacent_compatible_coords(coord: &GridCoord, grid: &Grid<Segment>) -> Vec<GridCoord> {
    let dirs: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    dirs.iter()
        .filter(|dir| {
            let next_coord = coord.get_relative_coord(**dir);
            match grid.get_with_coord(&next_coord) {
                Some(next_seg) => {
                    // Check if both current and next segments can connect to each other
                    grid.get_with_coord(coord).unwrap().can_connect(**dir)
                        && next_seg.can_connect(dir.opposite())
                }
                None => false,
            }
        })
        .map(|dir| coord.get_relative_coord(*dir))
        .collect_vec()
}
