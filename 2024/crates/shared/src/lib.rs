use std::{error::Error, fmt, ops::Add};

/// Represents a vector in 2D space
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// Constructor that creates a Point from a usize.
    /// 
    /// ### Panics
    /// This function does not perform bounds checks, take care to pass in appropriate sized values
    pub fn from_u(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    /// Creates a new point
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns the euclidean distance between this point and another point.
    pub fn euclid_dist(&self, b: Self) -> Self {
        Self {
            x: (b.x - self.x).abs(),
            y: (b.y - self.y).abs(),
        }
    }

    /// Scales the point by a given factor.
    /// 
    /// ### Panics
    /// Size bounds are not checked, take care to not pass in too large of a value
    pub fn scale_by(&self, factor: isize) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    /// Returns the components of this point as a tuple.
    pub fn components(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

/// Trait describing how to traverse a 1D array as a 2D array
pub trait Grid2D {
    fn row_len(&self) -> usize;
    fn col_len(&self) -> usize;

    /// Translates an array index into a point on the grid. Note that this does not do bounds-checks
    fn idx_to_coords(&self, idx: usize) -> Point {
        Point::from_u(idx % self.row_len(), idx / self.row_len())
    }

    /// Translates a coordinate into an array index. This does do bounds checks
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

/// Describes directions that can be traversed on a 2D grid.
/// Contains utilities to translate between a given direction and its basis unit vectors
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
    /// Translates a Direction into the basis vector used to indicate that direction
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

    /// Returns the direction 90 degrees clockwise of a given direction
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

    /// Returns the direction 180 degrees of a given direction
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

    /// Returns the direction 90 degrees counter-clockwise of a given direction
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

    /// Array of all 4 cardinal directions as unit vectors
    pub const CARDINALS: [Point; 4] = [
        Direction::NORTH.point(),
        Direction::SOUTH.point(),
        Direction::EAST.point(),
        Direction::WEST.point(),
    ];

    /// Array of all 8 cardinal and secondary directions as unit vectors
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
        let Some(newline_idx) = s.find('\n') else {
            return Err(GridError {
                kind: GridErrorType::NoRows,
            });
        };
        let row_len = newline_idx + 1;

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
