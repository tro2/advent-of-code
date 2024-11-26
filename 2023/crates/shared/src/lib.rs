#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
}

pub enum Offsets {
    None,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Offsets {
    pub const ALL: [Offset; 8] = [
        Offsets::North.to_offset(),
        Offsets::NorthEast.to_offset(),
        Offsets::East.to_offset(),
        Offsets::SouthEast.to_offset(),
        Offsets::South.to_offset(),
        Offsets::SouthWest.to_offset(),
        Offsets::West.to_offset(),
        Offsets::NorthWest.to_offset(),
    ];

    pub const fn to_offset(&self) -> Offset {
        match self {
            Offsets::None => Offset { x: 0, y: 0 },
            Offsets::North => Offset { x: 0, y: -1 },
            Offsets::NorthEast => Offset { x: 1, y: -1 },
            Offsets::East => Offset { x: 1, y: 0 },
            Offsets::SouthEast => Offset { x: 1, y: 1 },
            Offsets::South => Offset { x: 0, y: 1 },
            Offsets::SouthWest => Offset { x: -1, y: 1 },
            Offsets::West => Offset { x: -1, y: 0 },
            Offsets::NorthWest => Offset { x: -1, y: -1 },
        }
    }
}

pub trait Arrayto2DGrid {
    fn row_len(&self) -> usize;
    fn array_len(&self) -> usize;

    fn num_rows(&self) -> usize {
        self.array_len() / self.row_len()
    }

    fn idx_to_coords(&self, idx: usize) -> Option<Coordinates> {
        let x = idx % self.row_len();
        let y = idx / self.row_len();
        if y > self.num_rows() {
            return None;
        }
        Some(Coordinates { x, y })
    }

    fn coords_to_idx(&self, coords: &Coordinates) -> Option<usize> {
        if coords.x > self.row_len() || coords.y > self.num_rows() {
            return None;
        }
        Some(coords.y * self.row_len() + coords.x)
    }

    fn translate_coords(&self, coords: &Coordinates, offset: &Offset) -> Option<Coordinates> {
        let x = coords.x as isize + offset.x;
        let y = coords.y as isize + offset.y;
        if x < 0 || y < 0 || x > self.row_len() as isize || y > self.num_rows() as isize {
            return None;
        }
        Some(Coordinates { x: x as usize, y: y as usize })
    }

    fn translate_idx(&self, idx: usize, offset: &Offset) -> Option<usize> {
        let coords = self.idx_to_coords(idx)?;
        let ret = self.translate_coords(&coords, offset)?;
        self.coords_to_idx(&ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Grid {
        array_len: usize,
        row_len: usize,
    }

    impl Arrayto2DGrid for Grid {
        fn row_len(&self) -> usize {
            self.row_len
        }

        fn array_len(&self) -> usize {
            self.array_len
        }
    }

    #[test]
    fn test_idx_to_coords() {
        let grid_tall = Grid {
            array_len: 6,
            row_len: 2,
            // num_rows = 6 / 2 = 3
        };
        assert_eq!(grid_tall.idx_to_coords(0), Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(grid_tall.idx_to_coords(1), Some(Coordinates { x: 1, y: 0 }));
        assert_eq!(grid_tall.idx_to_coords(2), Some(Coordinates { x: 0, y: 1 }));
        assert_eq!(grid_tall.idx_to_coords(3), Some(Coordinates { x: 1, y: 1 }));
        assert_eq!(grid_tall.idx_to_coords(4), Some(Coordinates { x: 0, y: 2 }));
        assert_eq!(grid_tall.idx_to_coords(5), Some(Coordinates { x: 1, y: 2 }));

        let grid_wide = Grid {
            array_len: 6,
            row_len: 3,
            // num_rows = 6 / 3 = 2
        };
        assert_eq!(grid_wide.idx_to_coords(0), Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(grid_wide.idx_to_coords(1), Some(Coordinates { x: 1, y: 0 }));
        assert_eq!(grid_wide.idx_to_coords(2), Some(Coordinates { x: 2, y: 0 }));
        assert_eq!(grid_wide.idx_to_coords(3), Some(Coordinates { x: 0, y: 1 }));
        assert_eq!(grid_wide.idx_to_coords(4), Some(Coordinates { x: 1, y: 1 }));
        assert_eq!(grid_wide.idx_to_coords(5), Some(Coordinates { x: 2, y: 1 }));

        let grid_square = Grid {
            array_len: 9,
            row_len: 3,
            // num_rows = 9 / 3 = 3
        };
        assert_eq!(grid_square.idx_to_coords(0), Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(grid_square.idx_to_coords(1), Some(Coordinates { x: 1, y: 0 }));
        assert_eq!(grid_square.idx_to_coords(2), Some(Coordinates { x: 2, y: 0 }));
        assert_eq!(grid_square.idx_to_coords(3), Some(Coordinates { x: 0, y: 1 }));
        assert_eq!(grid_square.idx_to_coords(4), Some(Coordinates { x: 1, y: 1 }));
        assert_eq!(grid_square.idx_to_coords(5), Some(Coordinates { x: 2, y: 1 }));
        assert_eq!(grid_square.idx_to_coords(6), Some(Coordinates { x: 0, y: 2 }));
        assert_eq!(grid_square.idx_to_coords(7), Some(Coordinates { x: 1, y: 2 }));
        assert_eq!(grid_square.idx_to_coords(8), Some(Coordinates { x: 2, y: 2 }));
    }

    #[test]
    fn test_coords_to_idx() {
        let grid = Grid {
            array_len: 6,
            row_len: 2,
        };
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 0, y: 0 }), Some(0));
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 1, y: 0 }), Some(1));
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 0, y: 1 }), Some(2));
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 1, y: 1 }), Some(3));
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 0, y: 2 }), Some(4));
        assert_eq!(grid.coords_to_idx(&Coordinates { x: 1, y: 2 }), Some(5));
    }
}