mod grid;
use grid::{Grid, GridCoord};

fn main() {
    let grid = parse_grid(include_str!("../../../inputs/8-treetop-tree-house.txt"));
    dbg!(get_highest_scenic_score(grid));
}

fn visible_trees_in_dir(grid: &Grid<usize>, coord: GridCoord, (dx, dy): (isize, isize)) -> usize {
    // See `cells_in_line` in `get_visible_tree_count
    let line = (1..).map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dx * i)?,
            y: coord.y.checked_add_signed(dy * i)?,
        };
        // This is the main difference from `cells_in_line`
        // I believe it's returning the actual cell in `grid` as opposed to a copy of the cell
        Some(*grid.cell(coord)?)
    });

    let mut total = 0;
    let our_height = *grid.cell(coord).unwrap();
    for height in line {
        total += 1;
        // Note that we break after we increment. This takes care of adding the last tree
        if height >= our_height {
            break;
        }
    }
    total
}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    dirs.into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)))
        .product()
}

fn get_highest_scenic_score(grid: Grid<usize>) -> usize {
    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    all_coords
        .map(|coord| scenic_score(&grid, coord))
        .max()
        .unwrap()
}

fn get_visible_tree_count(grid: Grid<usize>) -> usize {
    // `flat_map` takes a 2d iterator and flattens it into a single 1d iterator
    // Basically this is taking a 2d vector of chars and turns it into a vector of GridCoord
    // We will be using `all_cords` as a list of keys to reference `grid`.
    let all_coords =
        (0..grid.height()).flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    // For every cell, iterate up, down, left, and right. If any of these four iterators has heights that are
    //   only decending, count the cell as visible.
    let num_visible_cells = all_coords
        .filter(|&coord| {
            let coord_height = grid.cell(coord).unwrap();
            // Define directions from original cell (West, East, North, South)
            let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
            deltas.iter().any(|&(dx, dy)| {
                // This yields an iterator that starts one away from coord and continues in that direction
                //   until it gets to the end of the valid coordinates in our grid
                let mut cells_in_line = (1..).map_while(|i| {
                    // This will yield a coordinate that is i spaces from the original coord
                    let coord = GridCoord {
                        x: coord.x.checked_add_signed(dx * i)?,
                        y: coord.y.checked_add_signed(dy * i)?,
                    };
                    // This checks to see if the coordinate is a valid one based on our grid
                    grid.cell(coord)
                });
                // return true if each cell in the line has a height less than the previous cell
                cells_in_line.all(|height| height < coord_height)
            })
        })
        .count();
    num_visible_cells
}

fn parse_grid(input: &str) -> Grid<usize> {
    // We want to store these variables seperately because it makes borrowing much easier.
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    // Create a blank grid
    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            // The * dereferences my cell, which in turn edits the cell directly.
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }

    grid
}
