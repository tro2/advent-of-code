#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Coordinate {
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
    pub fn to_offset(&self) -> Offset {
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

    fn idx_to_coords(&self, idx: usize) -> Option<Coordinate> {
        let x = idx % self.row_len();
        let y = idx / self.row_len();
        if y >= self.array_len() {
            return None;
        }
        Some(Coordinate { x, y })
    }

    fn coords_to_idx(&self, coords: Coordinate) -> Option<usize> {
        if coords.x >= self.row_len() || coords.y >= self.num_rows() {
            return None;
        }
        Some(coords.y * self.row_len() + coords.x)
    }

    fn translate_coords(&self, coords: Coordinate, offset: Offset) -> Option<Coordinate> {
        let x = coords.x as isize + offset.x;
        let y = coords.y as isize + offset.y;
        if x < 0 || y < 0 || x >= self.row_len() as isize || y >= self.num_rows() as isize {
            return None;
        }
        Some(Coordinate { x: x as usize, y: y as usize })
    }

    fn translate_idx(&self, idx: usize, offset: Offset) -> Option<usize> {
        let coords = self.idx_to_coords(idx)?;
        let ret = self.translate_coords(coords, offset)?;
        self.coords_to_idx(ret)
    }
}