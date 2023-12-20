use grid::{grid, Grid};

fn main() {
    let mut grid = parse_input(include_str!("ex1.txt"));
    print_grid(&grid);
    expand_grid(&mut grid);
    print_grid(&grid);
}

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = grid![];
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push_row(row)
    }
    grid
}

fn print_grid(grid: &Grid<char>) {
    for x in grid.iter_rows() {
        println!("{:?}", x.collect::<Vec<_>>());
    }
    println!();
}

fn expand_grid(grid: &mut Grid<char>) {
    // Add rows
    let mut i_empty_rows: Vec<usize> = Vec::new();
    grid.iter_rows().enumerate().for_each(|(i_row, row)| {
        if row.into_iter().all(|c| *c == '.') {
            i_empty_rows.push(i_row)
        }
    });
    println!("{:?}", i_empty_rows);
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
