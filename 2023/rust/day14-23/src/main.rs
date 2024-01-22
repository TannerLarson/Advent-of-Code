use core::panic;

use grid::Grid;
use itertools::Itertools;

// Can do 10000 cycles in a second or two (That's 100000 x less than the goal)
// TODO: Use Wes' advice

fn main() {
    let input = include_str!("ex1.txt");
    let num_cols = input.lines().next().unwrap().len();
    let mut platform = Platform(Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.chars().map(Rock::from))
            .collect_vec(),
        num_cols,
    ));
    platform.tilt(Direction::North);
    println!("Part 1: {}", platform.get_load(Direction::North));

    platform.spin(10000);
    println!("Part 2: {}", platform.get_load(Direction::North))
}

#[derive(PartialEq, Clone, Copy, PartialOrd, Eq, Ord, Debug)]
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

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct Platform(Grid<Rock>);

impl Platform {
    fn tilt(&mut self, dir: Direction) {
        let line_len = match dir {
            Direction::North | Direction::South => self.0.cols(),
            _ => self.0.rows(),
        };

        for i_line in 0..line_len {
            // The idea here is to have two indexes, i_counter and i_updater
            // i_counter will iterate until it finds a Cube rock, counting the number of round rocks it finds
            // i_updater will wait for i_counter to find a Cube rock. Once it does, it will update the values
            //   up to i_counter according to the number of round rocks found
            // Once i_updater has caught up to i_counter, reset num_round.
            let mut it_counter_coords = self.directional_range(dir, i_line).peekable();
            let mut it_updater_coords = self.directional_range(dir, i_line);
            let next = it_counter_coords.next().unwrap();
            let mut num_round = match self.0.get(next.0, next.1) {
                Some(Rock::Round) => 1,
                _ => 0,
            };
            let mut counter_updater_diff = 1;

            while let Some(counter_coord) = it_counter_coords.next() {
                counter_updater_diff += 1;
                let rock_at_counter = self.0.get(counter_coord.0, counter_coord.1);
                if *rock_at_counter.unwrap() == Rock::Round {
                    num_round += 1;
                }
                if it_counter_coords.peek().is_none() || *rock_at_counter.unwrap() == Rock::Cube {
                    // Use i_updater to update grid
                    let range = if it_counter_coords.peek().is_none()
                        && *rock_at_counter.unwrap() != Rock::Cube
                    {
                        0..counter_updater_diff
                    } else {
                        0..(counter_updater_diff - 1)
                    };

                    for i in range.clone() {
                        let updater_coord = it_updater_coords.next().unwrap();
                        let rock_at_updater =
                            self.0.get_mut(updater_coord.0, updater_coord.1).unwrap();
                        if *rock_at_updater == Rock::Cube {
                            continue;
                        } else if i >= range.len() - num_round {
                            *rock_at_updater = Rock::Round
                        } else {
                            *rock_at_updater = Rock::Empty
                        }
                    }
                    counter_updater_diff = 1;
                    num_round = 0;
                }
            }
        }
    }

    fn directional_range(
        &self,
        dir: Direction,
        i_line: usize,
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        match dir {
            Direction::North => Box::new((0..self.0.rows()).rev().map(move |i| (i, i_line))),
            Direction::East => Box::new((0..self.0.cols()).map(move |i| (i_line, i))),
            Direction::South => Box::new((0..self.0.rows()).map(move |i| (i, i_line))),
            Direction::West => Box::new((0..self.0.cols()).rev().map(move |i| (i_line, i))),
        }
    }

    fn spin(&mut self, num_cycles: usize) {
        // println!("Start");
        // print_grid(&self.0);
        for i_rotation in 0..(num_cycles * 4) {
            let dir = match i_rotation % 4 {
                0 => Direction::North,
                1 => Direction::West,
                2 => Direction::South,
                3 => Direction::East,
                _ => panic!("At the disco: {}", i_rotation % 4),
            };
            // println!("Direction: {:?}", dir);

            self.tilt(dir)
            // print_grid(&self.0);
        }
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
    println!()
}
