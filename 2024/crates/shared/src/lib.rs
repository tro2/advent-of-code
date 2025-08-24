use std::{error::Error, fmt, num::TryFromIntError, ops::Add};

/// Represents a vector in 2D space
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the euclidean distance between this point and another point.
    #[must_use]
    pub const fn euclid_dist(&self, b: Self) -> Self {
        Self {
            x: (b.x - self.x).abs(),
            y: (b.y - self.y).abs(),
        }
    }

    /// Scales the point by a given factor.
    /// 
    /// ### Panics
    /// Size bounds are not checked, take care to not pass in too large of a value
    #[must_use]
    pub const fn scale_by(&self, factor: i32) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    /// Returns the components of this point as a tuple.
    #[must_use]
    pub const fn components(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[must_use]
    pub fn to_index(&self, row_len: usize, col_len: usize) -> Option<usize> {
        let (x, y) = (self.x.try_into().ok()?, self.y.try_into().ok()?);
        if (0..row_len).contains(&x) && (0..col_len).contains(&y) {
            Some(x * row_len + x)
        } else {
            None
        }
    }
}

impl TryFrom<(usize, usize)> for Point {
    type Error = TryFromIntError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let (x, y) = value;
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

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
    #[must_use]
    pub const fn point(&self) -> Point {
        match self {
            Self::NORTH => Point { x: 0, y: -1 },
            Self::SOUTH => Point { x: 0, y: 1 },
            Self::EAST => Point { x: 1, y: 0 },
            Self::WEST => Point { x: -1, y: 0 },
            Self::NORTHEAST => Point { x: 1, y: -1 },
            Self::NORTHWEST => Point { x: -1, y: -1 },
            Self::SOUTHEAST => Point { x: 1, y: 1 },
            Self::SOUTHWEST => Point { x: -1, y: 1 },
        }
    }

    /// Returns the direction 90 degrees clockwise of a given direction
    #[must_use]
    pub const fn clockwise(&self) -> Self {
        match self {
            Self::NORTH => Self::EAST,
            Self::EAST => Self::SOUTH,
            Self::SOUTH => Self::WEST,
            Self::WEST => Self::NORTH,
            Self::NORTHWEST => Self::NORTHEAST,
            Self::NORTHEAST => Self::SOUTHEAST,
            Self::SOUTHEAST => Self::SOUTHWEST,
            Self::SOUTHWEST => Self::NORTHWEST,
        }
    }

    /// Returns the direction 180 degrees of a given direction
    #[must_use]
    pub const fn rev(&self) -> Self {
        match self {
            Self::NORTH => Self::SOUTH,
            Self::SOUTH => Self::NORTH,
            Self::EAST => Self::WEST,
            Self::WEST => Self::EAST,
            Self::NORTHEAST => Self::SOUTHWEST,
            Self::NORTHWEST => Self::SOUTHEAST,
            Self::SOUTHEAST => Self::NORTHWEST,
            Self::SOUTHWEST => Self::NORTHEAST,
        }
    }

    /// Returns the direction 90 degrees counter-clockwise of a given direction
    #[must_use]
    pub const fn counter_clockwise(&self) -> Self {
        match self {
            Self::NORTH => Self::WEST,
            Self::WEST => Self::SOUTH,
            Self::SOUTH => Self::EAST,
            Self::EAST => Self::NORTH,
            Self::NORTHWEST => Self::SOUTHWEST,
            Self::SOUTHWEST => Self::SOUTHEAST,
            Self::SOUTHEAST => Self::NORTHEAST,
            Self::NORTHEAST => Self::NORTHWEST,
        }
    }

    /// Array of all 4 cardinal directions as unit vectors
    pub const CARDINALS: [Point; 4] = [
        Self::NORTH.point(),
        Self::SOUTH.point(),
        Self::EAST.point(),
        Self::WEST.point(),
    ];

    pub const CARDINAL_DIRS: [Self; 4] = [
        Self::NORTH,
        Self::SOUTH,
        Self::EAST,
        Self::WEST,
    ];

    /// Array of all 8 cardinal and secondary directions as unit vectors
    pub const ALL: [Point; 8] = [
        Self::NORTH.point(),
        Self::SOUTH.point(),
        Self::EAST.point(),
        Self::WEST.point(),
        Self::NORTHEAST.point(),
        Self::NORTHWEST.point(),
        Self::SOUTHEAST.point(),
        Self::SOUTHWEST.point(),
    ];

    pub const ALL_DIRS: [Self; 8] = [
        Self::NORTH,
        Self::SOUTH,
        Self::EAST,
        Self::WEST,
        Self::NORTHEAST,
        Self::NORTHWEST,
        Self::SOUTHEAST,
        Self::SOUTHWEST,
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
    NoData
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
            GridErrorType::NoData => {
                write!(f, "no data found (only newlines)")
            }
        }
    }
}

pub struct DefaultGrid<'a> {
    pub data: &'a [u8],
    row_len: usize,
    col_len: usize,
}

impl DefaultGrid<'_> {
    /// Translates an array index into a point on the grid.
    #[must_use]
    fn idx_to_coords(&self, idx: usize) -> Option<Point> {
        Point::try_from((idx % self.row_len, idx / self.row_len)).ok()
    }

    /// Translates a coordinate into an array index.
    #[must_use]
    fn coord_to_idx(&self, coord: Point) -> Option<usize> {
        let (x, y) = (coord.x.try_into().ok()?, coord.x.try_into().ok()?);
        if (0..self.row_len).contains(&x) && (0..self.col_len).contains(&y) {
            Some(x * self.row_len + x)
        } else {
            None
        }
    }

    /// Returns an iterator over the bytes in the grid, excluding newlines, with absolute indices.
    #[must_use]
    pub const fn iter_cells(&'_ self) -> GridByteIter<'_> {
        GridByteIter {
            data: self.data,
            row_len: self.row_len,
            col_len: self.col_len,
            point: Point::new(0, 0)
        }
    }

    #[must_use]
    pub fn contains(&self, coord: Point) -> bool {
        self.coord_to_idx(coord).is_some()
    }

    #[must_use]
    pub fn at(&self, coord: Point) -> Option<u8> {
        self.coord_to_idx(coord).map(|idx| self.data[idx])
    }
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

        if row_len == 1 {
            return Err(GridError {
                kind: GridErrorType::NoData,
            });
        }

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

/// Iterator over `DefaultGrid` bytes excluding newlines, with real indices.
pub struct GridByteIter<'a> {
    data: &'a [u8],
    row_len: usize,
    col_len: usize,
    point: Point
}

impl Iterator for GridByteIter<'_> {
    type Item = (Point, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.point;
        let idx = self.point.to_index(self.row_len, self.col_len)?;
        let byte = self.data[idx];

        if byte == b'\n' {
            self.point = Point::new(0, self.point.y + 1);
            return self.next();
        }

        self.point.x += 1;
        return Some((curr, byte));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a = Point::new(-1, 0);
        let b = Point::new(2, 1);
        assert_eq!(a + b, Point::new(1, 1));
    }
}
