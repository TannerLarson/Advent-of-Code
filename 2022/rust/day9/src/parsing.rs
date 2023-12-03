use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))(i)
    }

    fn apply(self, coord: &mut GridCoord) {
        match self {
            Direction::Down => coord.y -= 1,
            Direction::Up => coord.y += 1,
            Direction::Left => coord.x -= 1,
            Direction::Right => coord.x += 1,
        };
    }
}

#[derive(Debug)]
pub(crate) struct Instruction {
    pub(crate) dir: Direction,
    pub(crate) dist: u32,
}

impl Instruction {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Direction::parse, tag(" "), nom::character::complete::u32),
            |(dir, dist)| Instruction { dir, dist },
        )(i)
    }
}

#[derive(Copy, Clone)]
struct GridCoord {
    x: i32,
    y: i32,
}

impl GridCoord {
    fn diff(self, other: &GridCoord) -> CoordDiff {
        CoordDiff {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

struct CoordDiff {
    x: i32,
    y: i32,
    distance: u32,
    transformation: Vec<Direction>,
}

impl CoordDiff {
    fn new(a: GridCoord, b: GridCoord) -> Self {
        let x = a.x - b.x;
        let y = a.y - b.y;
        let distance = x.abs().max(y.abs()) as u32;
        while x != 0 {}
        Self {
            x,
            y,
            distance,
            bigger,
            smaller,
        }
    }
}

pub(crate) struct Rope {
    head: GridCoord,
    tail: GridCoord,
}

impl Rope {
    pub(crate) fn new() -> Self {
        Self {
            head: GridCoord { x: 0, y: 0 },
            tail: GridCoord { x: 0, y: 0 },
        }
    }

    fn move_head(&mut self, dir: Direction) {
        dir.apply(&mut self.head);
    }

    fn update_tail(&mut self) {
        let diff = self.tail.diff(&self.head);
        if diff.distance() > 1 {
            return;
        }
    }
}
