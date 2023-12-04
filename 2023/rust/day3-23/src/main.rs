use utils::grid::{Grid, GridCoord};

struct PartNumber {
    number: u32,
    coords: Vec<GridCoord>,
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

fn get_part_numbers(grid: Grid<char>) -> Vec<PartNumber> {
    // For each row in the grid
    //   For each char in row
    //     if the char is a number
    //       iterate more chars until a non-digit is found or we get to the end of the row
    //       record each number found
    //       create a PartNumber from the info

    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    all_coords

    vec![PartNumber {
        number: 0,
        coords: vec![GridCoord { x: 0, y: 0 }],
    }]
}

fn main() {
    let input = include_str!("ex1.txt");
    let grid = parse_grid(input);
    get_part_numbers(grid);
}
