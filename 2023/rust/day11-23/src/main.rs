use core::fmt;

use grid::{grid, Grid};

fn main() {
    let mut grid = parse_input(include_str!("input.txt"));
    print_grid(&grid);
    // expand_grid(&mut grid);
    // let galaxy_coords = get_galaxy_coords(&grid);
    // let galaxy_pairs = generate_pairs(&galaxy_coords);
    // let ans: u64 = galaxy_pairs
    //     .iter()
    //     .map(|(a, b)| min_distance_between_galaxies(a, b) as u64)
    //     .sum();
    // println!("Part 1: {}", ans);

    mark_empty_lines(&mut grid);
    print_grid(&grid);
    let galaxy_coords = get_galaxy_coords(&grid);
    let galaxy_pairs = generate_pairs(&galaxy_coords);
    let ans: u64 = galaxy_pairs
        .iter()
        .map(|(a, b)| min_distance_between_galaxies(a, b, 1000000, &grid))
        .sum();
    println!("Part 2: {}", ans)
}

#[derive(Clone, Copy)]
struct GridCoord(usize, usize);

impl fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = grid![];
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push_row(row)
    }
    grid
}

// fn min_distance_between_galaxies(a: &GridCoord, b: &GridCoord) -> usize {
//     a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
// }

fn min_distance_between_galaxies(
    a: &GridCoord,
    b: &GridCoord,
    e_size: u64,
    grid: &Grid<char>,
) -> u64 {
    let mut passed_es = 0_usize;
    let diff_row = b.0 as i64 - a.0 as i64;
    let diff_col = b.1 as i64 - a.1 as i64;

    if diff_row >= 0 {
        for i in 0..diff_row {
            if *grid.get(a.0 as i64 + i, a.1).unwrap() == 'e' {
                passed_es += 1
            }
        }
    } else {
        for i in (diff_row..0).rev() {
            println!("{:?}", *grid.get(a.0 as i64 + i, a.0).unwrap());
            if *grid.get(a.0 as i64 + i, a.1).unwrap() == 'e' {
                passed_es += 1
            }
        }
    }
    if diff_col >= 0 {
        for i in 0..diff_col {
            if *grid.get(a.0 as i64 + diff_row, a.1 as i64 + i).unwrap() == 'e' {
                passed_es += 1
            }
        }
    } else {
        for i in (diff_col..0).rev() {
            if *grid.get(a.0 as i64 + diff_row, a.1 as i64 + i).unwrap() == 'e' {
                passed_es += 1
            }
        }
    }
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1) - passed_es) as u64
        + passed_es as u64 * e_size
}

fn generate_pairs(vec: &[GridCoord]) -> Vec<(GridCoord, GridCoord)> {
    vec.iter()
        .enumerate()
        .flat_map(|(i, &first)| {
            vec.iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(_, second)| (first, *second))
        })
        .collect()
}

fn get_galaxy_coords(grid: &Grid<char>) -> Vec<GridCoord> {
    grid.indexed_iter()
        .filter_map(|((i_row, i_col), c)| {
            if *c == '#' {
                Some(GridCoord(i_row, i_col))
            } else {
                None
            }
        })
        .collect()
}

fn print_grid(grid: &Grid<char>) {
    for x in grid.iter_rows() {
        println!("{:?}", x.collect::<Vec<_>>());
    }
    println!();
}

fn mark_empty_lines(grid: &mut Grid<char>) {
    for i in 0..grid.rows() {
        if grid.iter_row(i).all(|c| *c == '.') {
            grid.iter_row_mut(i).for_each(|c| *c = 'e')
        }
    }
    for i in 0..grid.cols() {
        if grid.iter_col(i).all(|c| *c == '.' || *c == 'e') {
            grid.iter_col_mut(i).for_each(|c| *c = 'e')
        }
    }
}

fn expand_grid(grid: &mut Grid<char>) {
    // Add rows
    let mut i_empty_rows: Vec<usize> = Vec::new();
    grid.iter_rows().enumerate().for_each(|(i_row, row)| {
        if row.into_iter().all(|c| *c == '.') {
            i_empty_rows.push(i_row)
        }
    });
    for i_row in i_empty_rows.iter().rev() {
        grid.insert_row(*i_row, (0..grid.cols()).map(|_| '.').collect())
    }

    // Add columns
    let mut i_empty_cols: Vec<usize> = Vec::new();
    grid.iter_cols().enumerate().for_each(|(i_col, col)| {
        if col.into_iter().all(|c| *c == '.') {
            i_empty_cols.push(i_col)
        }
    });
    for i_col in i_empty_cols.iter().rev() {
        grid.insert_col(*i_col, (0..grid.rows()).map(|_| '.').collect())
    }
}
