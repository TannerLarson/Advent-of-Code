use core::fmt;

use itertools::Itertools;
use utils::grid::{Grid, GridCoord};

struct PartNumber {
    number: u32,
    coords: Vec<GridCoord>,
    symbols: Vec<Symbol>,
}

impl fmt::Debug for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.number, self.coords)
    }
}

fn parse_grid(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid = Grid::new(width, height);

    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            *grid.cell_mut((x, y).into()).unwrap() = col
        }
    }
    grid
}

fn get_part_numbers(grid: &Grid<char>) -> Vec<PartNumber> {
    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    all_coords
        // Split coords into lines to cover the edge case where line 1 ends with a number and line 2 starts with a number
        .chunks(grid.width())
        .into_iter()
        .flat_map(|chunk| {
            chunk
                .group_by(|coord| grid.cell(*coord).unwrap().is_ascii_digit())
                .into_iter()
                .filter(|(is_digit, _)| *is_digit)
                .map(|(_, group)| group.collect::<Vec<GridCoord>>())
                .map(|coords| {
                    let number = coords
                        .iter()
                        .map(|coord| grid.cell(*coord).unwrap())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    PartNumber {
                        number,
                        coords: coords.clone(),
                        symbols: get_surrounding_symbols(coords.clone(), grid),
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn get_surrounding_symbols(coords: Vec<GridCoord>, grid: &Grid<char>) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    for coord in coords.clone() {
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                if let Some(cell) = grid.cell(coord.get_relative_coord(x_offset, y_offset)) {
                    if !cell.is_ascii_alphanumeric() && *cell != '.' {
                        symbols.push(Symbol {
                            symbol: *cell,
                            coord: coord.get_relative_coord(x_offset, y_offset),
                        });
                    }
                }
            }
        }
    }
    // Sort symbols in order to filter out duplicates
    symbols.sort_by(|a, b| {
        let x_comp = a.coord.x.cmp(&b.coord.x);
        if !x_comp.is_eq() {
            a.coord.y.cmp(&b.coord.y)
        } else {
            x_comp
        }
    });
    symbols.dedup();
    symbols
}

#[derive(Hash, Eq, PartialEq)]
struct Symbol {
    symbol: char,
    coord: GridCoord,
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.coord, self.symbol)
    }
}

// fn get_symbols(part_number: &PartNumber, grid: &Grid<char>) -> bool {}

fn main() {
    let input = include_str!("input.txt");
    let grid = parse_grid(input);
    let part_numbers = get_part_numbers(&grid);
    let mut sum_valid_part_numbers: u32 = part_numbers
        .iter()
        .filter(|part_number| !part_number.symbols.is_empty())
        .map(|part_number| part_number.number)
        .sum();
    println!("Part 1: {}", sum_valid_part_numbers);

    // part_numbers.iter().for_each(|part_number| {
    //     part_number
    //         .symbols
    //         .iter()
    //         .for_each(|symbol| println!("{symbol:?}"))
    // });

    let symbol_coord_to_part_number = part_numbers
        .iter()
        .flat_map(|part_number| {
            part_number
                .symbols
                .iter()
                .map(move |symbol| (symbol, part_number.number))
        })
        .into_group_map();

    // symbol_coord_to_part_number
    //     .iter()
    //     .for_each(|(key, value)| println!("{:?} / {:?}", key, value));

    sum_valid_part_numbers = symbol_coord_to_part_number
        .iter()
        .filter(|(key, value)| key.symbol == '*' && value.len() == 2)
        .map(|(_, value)| value.iter().product::<u32>())
        .sum();
    println!("Part 2: {}", sum_valid_part_numbers);
}
