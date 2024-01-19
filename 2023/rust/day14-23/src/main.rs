use core::panic;

use grid::Grid;
use itertools::Itertools;

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
    let tilted = &platform.tilted(Direction::North);
    println!("Part 1: {}", tilted.get_load(Direction::North));

    platform.spin(1);
    println!("Part 2: {}", platform.get_load(Direction::North))
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

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
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
        println!("Start");
        print_grid(&self.0);
        for i_rotation in 0..(num_cycles * 4) {
            let (dir, it_i_line) = match i_rotation % 4 {
                0 => (Direction::North, 0..self.0.cols()),
                1 => (Direction::East, 0..self.0.rows()),
                2 => (Direction::South, 0..self.0.cols()),
                3 => (Direction::West, 0..self.0.rows()),
                _ => panic!("At the disco: {}", i_rotation % 4),
            };
            println!("Direction: {:?}", dir);

            for i_line in it_i_line {
                // The idea here is to have two indexes, i_counter and i_updater
                // i_counter will iterate until it finds a Cube rock, counting the number of round rocks it finds
                // i_updater will wait for i_counter to find a Cube rock. Once it does, it will update the values
                //   up to i_counter according to the number of round rocks found
                // Once i_updater has caught up to i_counter, reset num_round.
                let mut num_round = 0;
                let mut counter_updater_diff = 0;

                for counter_coord in self.directional_range(dir, i_line) {
                    let mut it_updater_coord = self.directional_range(dir, i_line);
                    let rock_at_counter = self.0.get(counter_coord.0, counter_coord.1);
                    if rock_at_counter.is_none() || *rock_at_counter.unwrap() == Rock::Cube {
                        // Use i_updater to update grid
                        for _ in 0..counter_updater_diff {
                            let i_updater = it_updater_coord.next().unwrap();
                            if counter_coord.0.abs_diff(i_updater.0) <= num_round
                                && counter_coord.1.abs_diff(i_updater.1) <= num_round
                            {
                                *self.0.get_mut(i_updater.0, i_updater.1).unwrap() = Rock::Round
                            } else {
                                *self.0.get_mut(i_updater.0, i_updater.1).unwrap() = Rock::Empty
                            }
                        }
                        counter_updater_diff = 0;
                    } else if *rock_at_counter.unwrap() == Rock::Round {
                        num_round += 1;
                    }
                    counter_updater_diff += 1
                }
            }
            print_grid(&self.0);
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
