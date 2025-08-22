use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    ops::Add,
};

use shared::Point;

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let row_len = source.find('\n').unwrap() + 1;
    let grid = Grid {
        bytes: source.as_bytes(),
        row_len,
        col_len: source.len() / row_len,
    };

    let zeros = source
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, byte)| {
            if *byte == b'0' {
                return Some(idx);
            }
            None
        });

    let mut all = HashSet::new();

    let res = zeros
        .map(|start_idx| {
            let mut stack = vec![start_idx];
            let mut visited = HashSet::new();
            let mut end_count = 0;

            while let Some(curr) = stack.pop() {
                if !visited.contains(&curr) {
                    visited.insert(curr);

                    if grid.bytes[curr] == b'9' {
                        end_count += 1;
                    }

                    for idx in grid.next_idxs(curr) {
                        stack.push(idx);
                    }
                }
            }
            for idx in visited {
                all.insert(idx);
            }
            end_count
        })
        .sum();

    for (idx, byte) in source.as_bytes().iter().enumerate() {
        if *byte == b'\n' || all.contains(&idx) {
            print!("{}", *byte as char);
        } else {
            print!(".");
        }
    }

    res
}

pub fn part_02(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let row_len = source.find('\n').unwrap() + 1;
    let grid = Grid {
        bytes: source.as_bytes(),
        row_len,
        col_len: source.len() / row_len,
    };

    let zeros = source
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, byte)| {
            if *byte == b'0' {
                return Some(idx);
            }
            None
        });

    let res = zeros
        .map(|start_idx| {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut trail_ends = HashSet::new();
            let mut path_counts = vec![0; source.len()];

            queue.push_back(start_idx);
            path_counts[start_idx] = 1;

            while let Some(curr) = queue.pop_front() {
                print!("{} ", grid.bytes[curr] as char);
                if !visited.contains(&curr) {
                    visited.insert(start_idx);

                    if grid.bytes[curr] == b'9' {
                        trail_ends.insert(curr);
                    }

                    for idx in grid.next_idxs(curr) {
                        path_counts[idx] += 1;
                        queue.push_back(idx);
                    }
                }
            }
            println!();

            for (idx, byte) in source.as_bytes().iter().enumerate() {
                if *byte == b'\n' || path_counts[idx] == 0 {
                    print!("{}", *byte as char);
                } else {
                    print!("{}", path_counts[idx]);
                }
            }

            trail_ends.iter().map(|idx| path_counts[*idx]).sum::<u32>()
        })
        .sum();

    res
}

struct Grid<'a> {
    bytes: &'a [u8],
    row_len: usize,
    col_len: usize,
}

impl Grid<'_> {
    fn idx_to_coords(&self, idx: usize) -> Point {
        Point::from_u(idx % self.row_len, idx / self.row_len)
    }

    fn coord_to_idx(&self, coord: Point) -> Option<usize> {
        let (x, y) = coord.components();
        if (0..self.row_len as isize).contains(&x) && (0..self.col_len as isize).contains(&y) {
            Some((y * self.row_len as isize + x) as usize)
        } else {
            None
        }
    }

    fn next_idxs(&self, idx: usize) -> Vec<usize> {
        let offsets = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];
        let pos = self.idx_to_coords(idx);

        let curr_digit = (self.bytes[idx] as char).to_digit(10).unwrap();
        offsets
            .iter()
            .filter_map(|off| {
                if let Some(i) = self.coord_to_idx(pos.add(*off)) {
                    let ch = self.bytes[i] as char;
                    if let Some(digit) = ch.to_digit(10) {
                        if digit == curr_digit + 1 {
                            return Some(i);
                        }
                    }
                }
                None
            })
            .collect()
    }
}
