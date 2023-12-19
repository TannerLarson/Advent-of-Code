// This is a super simple struct, all it's doing is keeping track of x and y coordinates
// for a specific grid cell
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

impl GridCoord {
    pub fn get_relative_coord(self, x: i32, y: i32) -> Self {
        let new_x = self.x as i32 + x;
        let new_y = self.y as i32 + y;
        GridCoord {
            x: new_x as usize,
            y: new_y as usize,
        }
    }
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            // Instead of having a 2d vector, we simplify things by having a single vector
            // with each individual row strung one after the other.
            // This makes a lot of sense because it makes doing any column iteration
            // vs row iteration a simple math problem
            data: vec![T::default(); width * height],
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    pub fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn all_coords(&self) -> Vec<GridCoord> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| GridCoord::from((x, y))))
            .collect()
    }
}

pub fn parse_grid(input: &str) -> Grid<char> {
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
