use grid::Grid;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let num_cols = input.lines().next().unwrap().len();
    let platform = Platform(Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.chars().map(Rock::from))
            .collect_vec(),
        num_cols,
    ));
    // print_grid(&platform.0);
    println!();
    let tilted = &platform.tilted(Direction::North);
    print_grid(&tilted.0);
    println!("Part 1: {}", tilted.get_load(Direction::North))
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
enum Rock {
    Empty,
    Round,
    Cube,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => panic!("Value is bad: {}", value),
        }
    }
}

impl From<Rock> for char {
    fn from(value: Rock) -> Self {
        match value {
            Rock::Cube => '#',
            Rock::Round => 'O',
            Rock::Empty => '.',
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Platform(Grid<Rock>);

impl Platform {
    fn tilted(&self, dir: Direction) -> Self {
        // Point the tilt direction right
        let mut grid = self.0.clone();
        match dir {
            Direction::South => grid.rotate_left(),
            Direction::West => grid.rotate_half(),
            Direction::North => grid.rotate_right(),
            _ => (),
        }
        let tilted = grid
            .iter_rows()
            .flat_map(|row| {
                row.group_by(|&rock| rock != &Rock::Cube)
                    .into_iter()
                    .flat_map(|(_, group)| group.cloned().sorted())
                    .collect_vec()
            })
            .collect();

        let mut tilted = Grid::from_vec(tilted, self.0.cols());

        // Fix grid
        match dir {
            Direction::South => tilted.rotate_right(),
            Direction::West => tilted.rotate_half(),
            Direction::North => tilted.rotate_left(),
            _ => (),
        }
        Self(tilted)
    }

    fn get_load(&self, dir: Direction) -> usize {
        let mut grid = self.0.clone();
        // Orient grid so the selected direction is pointing south
        match dir {
            Direction::East => grid.rotate_right(),
            Direction::North => grid.rotate_half(),
            Direction::West => grid.rotate_left(),
            _ => (),
        }
        grid.iter_rows()
            .enumerate()
            .map(|(i, rocks)| rocks.filter(|&&rock| rock == Rock::Round).count() * (i + 1))
            .sum()
    }
}

fn print_grid(grid: &Grid<Rock>) {
    for row in grid.iter_rows() {
        for rock in row {
            print!("{:?} ", char::from(*rock))
        }
        println!()
    }
}
