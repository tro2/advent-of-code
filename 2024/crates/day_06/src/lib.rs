use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();
    let row_len = source.find('\n').unwrap() + 1;

    let grid = Grid {
        data: source.as_bytes(),
        row_len,
        col_len: source.len() / row_len,
        idx: source.find('^').unwrap(),
    };

    let result: HashMap<usize, Direction> = grid.iter().collect();

    // for (pos, byte) in source.as_bytes().iter().enumerate() {
    //     if let Some(c) = result.get(&pos) {
    //         if pos == source.find('^').unwrap() {
    //             print!("{}", '^');
    //         } else {
    //             print!("{}", c.to_display());
    //         }
    //     } else {
    //         print!("{}", *byte as char);
    //     }
    // }

    result.len() as u32
}

pub fn part_02(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();
    let row_len = source.find('\n').unwrap() + 1;
    let idx = source.find('^').unwrap();

    let grid = Grid {
        data: source.as_bytes(),
        row_len,
        col_len: source.len() / row_len,
        idx,
    };

    let mut outputs = HashSet::new();
    let mut copy = grid.data.to_owned();

    let mut result: HashMap<usize, Direction> = grid.iter().collect();
    result.remove(&idx);

    for (pos, _) in result {
        // replace pos with a #
        if grid.data[pos] == b'\n' {
            continue;
        }

        copy[pos] = b'#';

        let grid = Grid {
            data: &copy,
            ..grid
        };

        let mut visited = HashSet::new();
        for val in grid.iter() {
            if visited.contains(&val) {
                outputs.insert(pos);
                visited.clear();
                break;
            }
            visited.insert(val);
        }

        copy[pos] = b'.';
    }

    outputs.remove(&idx);

    outputs.len() as u32
}

struct Grid<'a> {
    data: &'a [u8],
    row_len: usize,
    col_len: usize,
    idx: usize,
}

impl Grid<'_> {
    fn iter(&'_ self) -> GridIter<'_> {
        let x = self.idx % self.row_len;
        let y = self.idx / self.row_len;
        GridIter {
            grid: self,
            pos: (x as isize, y as isize),
            direction: Direction::North,
            // visited: HashSet::new()
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_coord(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    // fn to_display(&self) -> char {
    //     match self {
    //         Direction::North | Direction::South => '|',
    //         Direction::East | Direction::West => '-',
    //     }
    // }
}

#[derive(Clone)]
struct GridIter<'a> {
    grid: &'a Grid<'a>,
    pos: (isize, isize),
    direction: Direction,
    // visited: HashSet<(usize, Direction)>
}

impl Iterator for GridIter<'_> {
    type Item = (usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let curr_dir = self.direction;

        while self.path_blocked() {
            self.direction = self.next_dir();
        }

        if let Some(result) = self.coord_to_idx(self.pos, true) {
            self.pos = self.next_coords();
            return std::option::Option::Some((result, curr_dir));
        }

        None
    }
}

impl GridIter<'_> {
    fn next_dir(&self) -> Direction {
        match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn path_blocked(&self) -> bool {
        if let Some(idx) = self.coord_to_idx(self.next_coords(), false) {
            return self.grid.data[idx] == b'#';
        }
        false
    }

    fn next_coords(&self) -> (isize, isize) {
        let (x, y) = self.pos;
        let dir = self.direction.to_coord();
        (x + dir.0, y + dir.1)
    }

    fn coord_to_idx(&self, (x, y): (isize, isize), check_exit: bool) -> Option<usize> {
        let len = self.grid.row_len as isize;
        let height = self.grid.col_len as isize;
        if (0..len).contains(&x) && (0..height).contains(&y) {
            let idx = y * len + x;

            if check_exit && self.grid.data[idx as usize] == b'\n' {
                return None;
            }

            return Some(idx as usize);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = concat!(
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n"
    );

    #[test]
    fn idx_tester() {
        let guard_idx = DATA.find('^').unwrap();

        let grid = Grid {
            data: DATA.as_bytes(),
            row_len: 11,
            col_len: 10,
            idx: guard_idx,
        };

        assert_eq!(grid.iter().coord_to_idx((4, 6), false).unwrap(), guard_idx)
    }

    #[test]
    fn simple_exit() {
        let grid = Grid {
            data: DATA.as_bytes(),
            row_len: 11,
            col_len: 10,
            idx: 5,
        };

        let mut iter = grid.iter();
        iter.direction = Direction::East;

        assert_eq!(iter.collect::<HashSet<(usize, Direction)>>().len(), 5)
    }
}
