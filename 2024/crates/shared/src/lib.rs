use std::{error::Error, fmt, ops::Add};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn from_u(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn euclid_dist(&self, b: Self) -> Self {
        Self {
            x: (b.x - self.x).abs(),
            y: (b.y - self.y).abs(),
        }
    }

    pub fn scale_by(&self, factor: isize) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn components(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

pub trait Grid2D {
    fn row_len(&self) -> usize;
    fn col_len(&self) -> usize;

    fn idx_to_coords(&self, idx: usize) -> Point {
        Point::from_u(idx % self.row_len(), idx / self.row_len())
    }

    fn coord_to_idx(&self, coord: Point) -> Option<usize> {
        let (x, y) = coord.components();
        if (0..self.row_len() as isize).contains(&x) && (0..self.col_len() as isize).contains(&y) {
            Some((y * self.row_len() as isize + x) as usize)
        } else {
            None
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        let dir = rhs.point();
        Self::Output {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}

impl Direction {
    pub const fn point(&self) -> Point {
        match self {
            Direction::NORTH => Point { x: 0, y: -1 },
            Direction::SOUTH => Point { x: 0, y: 1 },
            Direction::EAST => Point { x: 1, y: 0 },
            Direction::WEST => Point { x: -1, y: 0 },
            Direction::NORTHEAST => Point { x: 1, y: -1 },
            Direction::NORTHWEST => Point { x: -1, y: -1 },
            Direction::SOUTHEAST => Point { x: 1, y: 1 },
            Direction::SOUTHWEST => Point { x: -1, y: 1 },
        }
    }

    pub const fn cw_card(&self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
            Direction::NORTHWEST => Direction::NORTHEAST,
            Direction::NORTHEAST => Direction::SOUTHEAST,
            Direction::SOUTHEAST => Direction::SOUTHWEST,
            Direction::SOUTHWEST => Direction::NORTHWEST,
        }
    }

    pub const fn rev(&self) -> Self {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::NORTH,
            Direction::EAST => Direction::WEST,
            Direction::WEST => Direction::EAST,
            Direction::NORTHEAST => Direction::SOUTHWEST,
            Direction::NORTHWEST => Direction::SOUTHEAST,
            Direction::SOUTHEAST => Direction::NORTHWEST,
            Direction::SOUTHWEST => Direction::NORTHEAST,
        }
    }

    pub const fn ccw_card(&self) -> Self {
        match self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
            Direction::NORTHWEST => Direction::SOUTHWEST,
            Direction::SOUTHWEST => Direction::SOUTHEAST,
            Direction::SOUTHEAST => Direction::NORTHEAST,
            Direction::NORTHEAST => Direction::NORTHWEST,
        }
    }

    // pub const NORTH: Point = Point { x: 0, y: -1 };
    // pub const SOUTH: Point = Point { x: 0, y: 1 };
    // pub const EAST: Point = Point { x: 1, y: 0 };
    // pub const WEST: Point = Point { x: -1,y:  0 };
    // pub const NORTHEAST: Point = Point { x: 1, y: -1 };
    // pub const NORTHWEST: Point = Point { x: -1,y: -1 };
    // pub const SOUTHEAST: Point = Point { x: 1, y: 1 };
    // pub const SOUTHWEST: Point = Point { x: -1,y:  1 };

    pub const CARDINALS: [Point; 4] = [
        Direction::NORTH.point(),
        Direction::SOUTH.point(),
        Direction::EAST.point(),
        Direction::WEST.point(),
    ];

    pub const ALL: [Point; 8] = [
        Direction::NORTH.point(),
        Direction::SOUTH.point(),
        Direction::EAST.point(),
        Direction::WEST.point(),
        Direction::NORTHEAST.point(),
        Direction::NORTHWEST.point(),
        Direction::SOUTHEAST.point(),
        Direction::SOUTHWEST.point(),
    ];
}

#[derive(Debug)]
pub struct GridError {
    kind: GridErrorType,
}

#[derive(Debug)]
enum GridErrorType {
    NotSquare,
    NoRows,
}

impl Error for GridError {}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            GridErrorType::NotSquare => {
                write!(f, "array cannot be squared")
            }
            GridErrorType::NoRows => {
                write!(f, "no rows (newlines) found")
            }
        }
    }
}

pub struct DefaultGrid<'a> {
    pub data: &'a [u8],
    row_len: usize,
    col_len: usize,
}

impl<'a> TryFrom<&'a str> for DefaultGrid<'a> {
    type Error = GridError;

    fn try_from(s: &'a str) -> Result<Self, GridError> {
        let n_pos = s.find('\n');
        if n_pos.is_none() {
            return Err(GridError {
                kind: GridErrorType::NoRows,
            });
        }
        let row_len = n_pos.unwrap() + 1;

        if s.len() % row_len != 0 {
            return Err(GridError {
                kind: GridErrorType::NotSquare,
            });
        }
        let col_len = s.len() / row_len;

        Ok(DefaultGrid {
            data: s.as_bytes(),
            row_len,
            col_len,
        })
    }
}

impl Grid2D for DefaultGrid<'_> {
    fn row_len(&self) -> usize {
        self.row_len
    }

    fn col_len(&self) -> usize {
        self.col_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a = Point::new(-1, 0);
        let b = Point::from_u(2, 1);
        assert_eq!(a + b, Point::new(1, 1));
    }
}
