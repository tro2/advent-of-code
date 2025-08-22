use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use shared::{DefaultGrid, Direction, Grid2D, Point};

pub fn part_01(path: &str) -> usize {
    let source = read_to_string(path).unwrap();

    let mut garden = Garden {
        grid: DefaultGrid::try_from(source.as_str()).unwrap(),
        idx_to_plot: HashMap::new(),
        curr_plot_id: 0,
    };

    let mut sum = 0;

    for (idx, byte) in garden.grid.data.iter().enumerate() {
        if *byte == b'\n' || garden.idx_to_plot.contains_key(&idx) {
            continue;
        }

        garden.idx_to_plot.insert(idx, garden.curr_plot_id);
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = Vec::new();
        queue.push(idx);
        while !queue.is_empty() {
            let idx = queue.pop().unwrap();

            let idxs = garden.check_surroundings(idx, *byte);

            area += 1;
            perimeter += 4 - idxs.len();

            for i in idxs {
                if !garden.idx_to_plot.contains_key(&i) {
                    // new point on plot found
                    garden.idx_to_plot.insert(i, garden.curr_plot_id);
                    queue.push(i);
                }
            }
        }

        garden.curr_plot_id += 1;

        sum += area * perimeter;
    }

    sum
}

pub fn part_02(path: &str) -> usize {
    let source = read_to_string(path).unwrap();

    let mut m = ComplexGarden {
        garden: Garden {
            grid: DefaultGrid::try_from(source.as_str()).unwrap(),
            idx_to_plot: HashMap::new(),
            curr_plot_id: 0,
        },
        plot_to_idxs: HashMap::new(),
        plot_to_info: HashMap::new(),
    };

    for (idx, byte) in m.garden.grid.data.iter().enumerate() {
        if *byte == b'\n' || m.garden.idx_to_plot.contains_key(&idx) {
            continue;
        }

        m.garden.idx_to_plot.insert(idx, m.garden.curr_plot_id);
        m.plot_to_idxs
            .insert(m.garden.curr_plot_id, HashSet::from([idx]));
        let mut area = 0;
        let mut num_sides = 0;
        let mut queue = Vec::new();
        queue.push(idx);
        while !queue.is_empty() {
            let idx = queue.pop().unwrap();

            let idxs = m.garden.check_surroundings(idx, *byte);

            area += 1;

            for i in idxs {
                if !m.garden.idx_to_plot.contains_key(&i) {
                    // new point on plot found
                    m.garden.idx_to_plot.insert(i, m.garden.curr_plot_id);
                    m.plot_to_idxs
                        .get_mut(&m.garden.curr_plot_id)
                        .unwrap()
                        .insert(i);
                    queue.push(i);
                }
            }
        }

        num_sides += m.garden.traverse_fence(idx);

        m.plot_to_info.insert(
            m.garden.curr_plot_id,
            GardenInfo {
                area,
                side_count: num_sides,
            },
        );

        m.garden.curr_plot_id += 1;
    }

    m.plot_to_info
        .iter()
        .map(|(_, &info)| info.area * info.side_count)
        .sum()
}

struct Garden<'a> {
    grid: DefaultGrid<'a>,
    idx_to_plot: HashMap<usize, u32>,
    curr_plot_id: u32,
}

impl Garden<'_> {
    fn at(&self, coord: Point) -> Option<u8> {
        if let Some(idx) = self.grid.coord_to_idx(coord) {
            return Some(self.grid.data[idx]);
        }
        None
    }

    fn check_surroundings(&mut self, idx: usize, key: u8) -> Vec<usize> {
        let pos = self.grid.idx_to_coords(idx);
        Direction::CARDINALS
            .iter()
            .filter_map(|&offset| {
                let coord = pos + offset;
                if let Some(curr) = self.grid.coord_to_idx(coord) {
                    if self.grid.data[curr] == key {
                        return Some(curr);
                    }
                }
                None
            })
            .collect()
    }

    fn traverse_fence(&self, corner: usize) -> usize {
        let key = self.grid.data[corner];
        let start = self.grid.idx_to_coords(corner) + Direction::NORTH;

        let mut pos = start;
        let mut sides = 0;
        let mut curr_dir = Direction::EAST;

        loop {
            let (next_dir, side_count) = self.path_blocked_cw(pos, curr_dir, key);
            sides += side_count;
            curr_dir = next_dir;
            pos = pos + next_dir;

            if pos == start {
                break;
            }
        }

        sides
    }

    #[cfg(test)]
    fn traverse_fence_dbg(&self, corner: usize) -> (usize, Vec<usize>, Vec<Direction>, Vec<Point>) {
        let key = self.grid.data[corner];
        let start = self.grid.idx_to_coords(corner) + Direction::NORTH;

        let mut pos = start;
        let mut sides = 0;
        let mut curr_dir = Direction::EAST;
        let mut side_counts = Vec::new();
        let mut dirs = Vec::new();
        let mut positions = vec![pos];

        loop {
            let (next_dir, side_count) = self.path_blocked_cw(pos, curr_dir, key);
            side_counts.push(side_count);
            dirs.push(next_dir);
            sides += side_count;
            curr_dir = next_dir;
            pos = pos + next_dir;
            positions.push(pos);

            if pos == start {
                break;
            }
        }

        (sides, side_counts, dirs, positions)
    }

    fn path_blocked_cw(&self, pos: Point, dir: Direction, key: u8) -> (Direction, usize) {
        let result = self.at(pos + dir.cw_card());
        if result.is_none() || result.unwrap() != key {
            return (dir.cw_card(), 1);
        }

        let result = self.at(pos + dir);
        if result.is_none() || result.unwrap() != key {
            return (dir, 0);
        }

        let result = self.at(pos + dir.ccw_card());
        if result.is_none() || result.unwrap() != key {
            return (dir.ccw_card(), 1);
        }

        (dir.rev(), 2)
    }

    fn path_inside_blocked_cw(&self, pos: Point, dir: Direction) -> (Direction, usize) {
        let key = self.at(pos).unwrap();
        if let Some(byte) = self.at(pos + dir.ccw_card()) {
            if byte == key {
                return (dir.ccw_card(), 1);
            }
        }

        if let Some(byte) = self.at(pos + dir) {
            if byte == key {
                return (dir, 0);
            }
        }

        let key = self.at(pos).unwrap();
        if let Some(byte) = self.at(pos + dir.cw_card()) {
            if byte == key {
                return (dir.cw_card(), 1);
            }
        }

        (dir.rev(), 2)
    }

    fn traverse_inside(&self, corner: usize) -> (usize, Vec<u8>) {
        let start = self.grid.idx_to_coords(corner);

        let mut pos = start;
        let mut sides = 0;
        let mut curr_dir = Direction::EAST;
        let mut external_plots = Vec::new();

        loop {
            let (next_dir, side_count) = self.path_inside_blocked_cw(pos, curr_dir);
            sides += side_count;
            curr_dir = next_dir;
            pos = pos + next_dir.point();

            if pos == start {
                break;
            }
        }

        (sides, external_plots)
    }
}

struct ComplexGarden<'a> {
    garden: Garden<'a>,
    plot_to_idxs: HashMap<u32, HashSet<usize>>,
    plot_to_info: HashMap<u32, GardenInfo>,
}

#[derive(Clone, Copy)]
struct GardenInfo {
    area: usize,
    side_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_test_straight() {
        let source = "AAAA\n";
        let garden = Garden {
            grid: DefaultGrid::try_from(source).unwrap(),
            idx_to_plot: HashMap::new(),
            curr_plot_id: 0,
        };

        let (out_dir, side_count) =
            garden.path_blocked_cw(Point::new(0, -1), Direction::EAST, b'A');

        assert_eq!(out_dir, Direction::EAST);
        assert_eq!(side_count, 0);

        let (out_dir, side_count) =
            garden.path_blocked_cw(Point::new(3, -1), Direction::EAST, b'A');

        assert_eq!(out_dir, Direction::EAST);
        assert_eq!(side_count, 0);
    }

    #[test]
    fn dir_test_down() {
        let source = "AAAA\n";
        let garden = Garden {
            grid: DefaultGrid::try_from(source).unwrap(),
            idx_to_plot: HashMap::new(),
            curr_plot_id: 0,
        };

        let (out_dir, side_count) =
            garden.path_blocked_cw(Point::new(4, -1), Direction::EAST, b'A');

        assert_eq!(out_dir, Direction::SOUTH);
        assert_eq!(side_count, 1);
    }

    #[test]
    fn traverse_simple() {
        let source = "AAAA\n";
        let garden = Garden {
            grid: DefaultGrid::try_from(source).unwrap(),
            idx_to_plot: HashMap::from([(0, 0), (1, 0), (3, 0), (4, 0)]),
            curr_plot_id: 0,
        };

        let traverse = garden.traverse_fence(0);

        assert_eq!(traverse, 4);
    }

    #[test]
    fn traverse_complex() {
        let source = "AABA\nBAAA\n";
        // AABA
        // BAAA
        let zero = [0_usize, 1, 3, 6, 7, 8].iter().map(|&idx| (idx, 0_u32));
        let one = [2].iter().map(|&idx| (idx, 1));
        let two = [5].iter().map(|&idx| (idx, 2));
        let garden = Garden {
            grid: DefaultGrid::try_from(source).unwrap(),
            idx_to_plot: zero.chain(one).chain(two).collect(),
            curr_plot_id: 0,
        };

        let expected_counts = vec![0, 0, 1, 2, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1];
        let expected_dirs = vec![
            Direction::EAST,
            Direction::EAST,
            Direction::SOUTH,
            Direction::NORTH,
            Direction::EAST,
            Direction::EAST,
            Direction::SOUTH,
            Direction::SOUTH,
            Direction::SOUTH,
            Direction::WEST,
            Direction::WEST,
            Direction::WEST,
            Direction::WEST,
            Direction::NORTH,
            Direction::WEST,
            Direction::NORTH,
            Direction::NORTH,
            Direction::EAST,
        ];
        let expected_pos = vec![
            Point::new(0, -1),
            Point::new(1, -1),
            Point::new(2, -1),
            Point::new(2, 0),
            Point::new(2, -1),
            Point::new(3, -1),
            Point::new(4, -1),
            Point::new(4, 0),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
            Point::new(0, 2),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, -1),
        ];

        let (sides, counts, dirs, pos) = garden.traverse_fence_dbg(0);

        assert_eq!(dirs, expected_dirs);
        assert_eq!(counts, expected_counts);
        assert_eq!(pos, expected_pos);
        assert_eq!(sides, 11);
    }

    #[test]
    fn internal_trav() {
        let source = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA\n";
        // AAAAAA
        // AAABBA
        // AAABBA
        // ABBAAA
        // ABBAAA
        // AAAAAA
        let zero = [0_usize, 1, 3, 6, 7, 8].iter().map(|&idx| (idx, 0_u32));
        let one = [11, 12, 19, 20].iter().map(|&idx| (idx, 1));
        let two = [25, 26, 33, 34].iter().map(|&idx| (idx, 2));

        let garden = Garden {
            grid: DefaultGrid::try_from(source).unwrap(),
            idx_to_plot: zero.chain(one).chain(two).collect(),
            curr_plot_id: 0,
        };

        let output = garden.traverse_fence_dbg(0);
        assert_eq!(output.0, 12);
    }
}
