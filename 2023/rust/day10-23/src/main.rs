use core::{fmt, panic};
use grid::{grid, Grid};
use itertools::Itertools;

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    let start = match grid
        .indexed_iter()
        .filter_map(|(coord, seg)| {
            if seg.directions.len() == 4 {
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

    let mut pipe = Pipe::build(start, &grid).unwrap();

    println!("{:?}", pipe);
    println!("Part 1: {}", pipe.0.len() / 2);

    let new_symbol = extrapolate_s_symbol(&pipe);
    if let Some(first_element) = pipe.0.first_mut() {
        first_element.symbol = new_symbol;
    } else {
        panic!("haoiest")
    }
    println!("{:?}", pipe);
}

trait GridExt<T> {
    fn get_with_coord(&self, coord: &GridCoord) -> Option<&T>;

    fn get_mut_with_coord(&mut self, coord: &GridCoord) -> Option<&mut T>;
}

impl<T> GridExt<T> for Grid<T> {
    fn get_with_coord(&self, coord: &GridCoord) -> Option<&T> {
        self.get(coord.x, coord.y)
    }

    fn get_mut_with_coord(&mut self, coord: &GridCoord) -> Option<&mut T> {
        self.get_mut(coord.x, coord.y)
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

#[derive(PartialEq, Clone, Copy, Debug, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
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

#[derive(Clone)]
struct Segment {
    symbol: char,
    directions: Vec<Direction>,
    coord: GridCoord,
}

impl Segment {
    fn new(symbol: char, coord: GridCoord) -> Self {
        let directions = match symbol {
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
            _ => panic!("Trying to parse bad character: {}", symbol),
        };

        Self {
            symbol,
            directions,
            coord,
        }
    }

    fn can_connect(&self, direction: Direction) -> bool {
        self.directions.iter().any(|dir| *dir == direction)
    }

    fn is_enclosed(&self, grid: Grid<Segment>) -> bool {
        if self.symbol != '.' {
            return false;
        }
        let mut current_seg = self.clone();
        let mut total = 0_u32;
        let mut saw_f = false;
        let mut saw_seven = false;
        while current_seg.coord.y != 0 {
            match current_seg.symbol {
                '_' => total += 1,
                'F' => saw_f = true,
                '7' => saw_seven = true,
                'L' => {
                    if saw_f {
                        total += 1
                    }
                    saw_f = false
                }
                'J' => {
                    if saw_seven {
                        total += 1
                    }
                    saw_seven = false
                }
                _ => (),
            }
            current_seg = grid
                .get_with_coord(&current_seg.coord.get_relative_coord(Direction::South))
                .unwrap()
                .clone()
        }
        total % 2 == 1
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {:?}", self.symbol, self.coord, self.directions)
    }
}

struct Pipe(Vec<Segment>);

impl Pipe {
    fn build(coord_start: GridCoord, grid: &Grid<Segment>) -> Option<Self> {
        let mut pipe_coords: Vec<GridCoord> = Vec::new();
        let mut current_coord = coord_start;
        let mut prev_coord: Option<GridCoord> = None;

        loop {
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
                .map(|coord| grid.get_with_coord(&coord).cloned().unwrap())
                .collect_vec(),
        );
        Some(pipe)
    }
}

impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Pipe [")?;
        for seg in &self.0 {
            writeln!(f, "    {:?}", seg)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

fn extrapolate_s_symbol(pipe: &Pipe) -> char {
    let mut directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    directions = directions
        .iter()
        .filter(|dir| {
            let coord = pipe.0.first().unwrap().coord.get_relative_coord(**dir);
            pipe.0.last().unwrap().coord == coord || pipe.0.get(1).unwrap().coord == coord
        })
        .cloned()
        .collect_vec();
    directions.sort();
    match directions[..] {
        [Direction::North, Direction::South] => '|',
        [Direction::North, Direction::East] => 'F',
        [Direction::North, Direction::West] => 'J',
        [Direction::East, Direction::South] => 'L',
        [Direction::East, Direction::West] => '-',
        [Direction::South, Direction::West] => '7',
        _ => panic!("We shouldn't get here: {:?}", directions),
    }
}

fn parse_input(input: &str) -> Grid<Segment> {
    let mut grid = grid![];
    for (i_line, line) in input.lines().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(i_char, c)| Segment::new(c, GridCoord::new((i_line, i_char))))
            .collect_vec();
        grid.push_row(row)
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
