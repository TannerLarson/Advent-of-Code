use core::fmt;
use std::cmp::min;

use grid::Grid;
use itertools::Itertools;

/// 1. For each row, count the number of changes that would need to happen for each half to mirror the other
/// 3. If that number ever goes above 1, return early

fn main() {
    let input = include_str!("input.txt");
    let grids = input.split("\n\n").map(parse_grid).collect_vec();
    // print_grid(grids.first().unwrap());
    let ans = grids
        .iter()
        .map(|grid| match get_reflection_point(grid) {
            Direction::Horizontal(i) => i as u32,
            Direction::Vertical(i) => (i * 100) as u32,
        })
        .sum::<u32>();
    println!("Part 1: {:?}", ans);
}

#[derive(Default, PartialEq, Clone)]
enum Object {
    #[default]
    Ash,
    Rock,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Object::Ash,
            '#' => Object::Rock,
            _ => panic!("Bad char: {}", value),
        }
    }
}

impl From<&Object> for char {
    fn from(value: &Object) -> Self {
        match value {
            Object::Ash => '.',
            Object::Rock => '#',
        }
    }
}

fn parse_grid(input: &str) -> Grid<Object> {
    let num_cols = input.lines().next().unwrap().len();
    let rows = input
        .lines()
        .flat_map(|line| line.chars().map(Object::from))
        .collect_vec();

    Grid::from_vec(rows, num_cols)
}

fn print_grid(grid: &Grid<Object>) {
    for row in grid.iter_rows() {
        for object in row {
            print!("{:?} ", object)
        }
        println!()
    }
}

enum Direction {
    Vertical(usize),
    Horizontal(usize),
}

fn get_reflection_point(grid: &Grid<Object>) -> Direction {
    if let Some(i) = vertical_reflection_point(grid, 1) {
        Direction::Horizontal(i)
    } else {
        let mut new_grid: Grid<Object> = grid.clone();
        new_grid.rotate_left();
        Direction::Vertical(vertical_reflection_point(&new_grid, 1).unwrap())
    }
}

// // Gets the reflection point of a grid
// fn vertical_reflection_point(grid: &Grid<Object>) -> Option<usize> {
//     (1..grid.cols()).find(|&i| {
//         is_valid_reflection_point(grid.iter_row(0).collect(), i)
//             && grid
//                 .iter_rows()
//                 .skip(1)
//                 .all(|row| is_valid_reflection_point(row.collect(), i))
//     })
// }

fn vertical_reflection_point(grid: &Grid<Object>, expected_diffs: usize) -> Option<usize> {
    for i in 1..grid.cols() {
        let mut num_changes = 0;
        let mut j = 0;
        while num_changes <= 1 && j < grid.rows() {
            num_changes +=
                num_changes_necessary_to_be_valid_reflection_point(grid.iter_row(j).collect(), i);
            j += 1;
        }
        if num_changes == expected_diffs {
            return Some(i);
        }
    }
    None
}

fn num_changes_necessary_to_be_valid_reflection_point(line: Vec<&Object>, point: usize) -> usize {
    let l = &line[..point];
    let r = &line[point..];
    let shorter_len = min(l.len(), r.len());

    l.iter()
        .rev()
        .take(shorter_len)
        .zip(r.iter().take(shorter_len))
        .filter(|(l, r)| r != l)
        .count()
}

fn is_valid_reflection_point(line: Vec<&Object>, point: usize) -> bool {
    let l = &line[..point];
    let r = &line[point..];
    let shorter_len = min(l.len(), r.len());

    l.iter()
        .rev()
        .take(shorter_len)
        .zip(r.iter().take(shorter_len))
        .all(|(l, r)| r == l)
}
