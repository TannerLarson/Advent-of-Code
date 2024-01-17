use grid::Grid;
use itertools::{sorted, Itertools};

fn main() {
    let input = include_str!("ex1.txt");
    let num_cols = input.lines().next().unwrap().len();
    let grid = Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.chars().map(Rock::from))
            .collect_vec(),
        num_cols,
    );
    print_grid(&grid)
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
    fn tilted(&mut self, dir: Direction) -> Grid<Rock> {
        // Point the tilt direction right
        match dir {
            Direction::South => self.0.rotate_left(),
            Direction::West => self.0.rotate_half(),
            Direction::North => self.0.rotate_right(),
            _ => (),
        }
        let tilted = self
            .0
            .iter_rows()
            .flat_map(|row| {
                row.group_by(|&rock| rock != &Rock::Cube)
                    .into_iter()
                    .flat_map(|(_, group)| group.cloned().sorted())
                    .collect_vec()
            })
            .collect();

        // Fix grid
        match dir {
            Direction::South => self.0.rotate_right(),
            Direction::West => self.0.rotate_half(),
            Direction::North => self.0.rotate_left(),
            _ => (),
        }

        Grid::from_vec(tilted, self.0.cols())
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
