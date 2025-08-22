use shared::Point;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Add,
};

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let row_len = source.find('\n').unwrap() + 1; // deliberate, newlines will not factor into coord calcs
    let col_len = source.len() / row_len;
    let mut antennaes: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (pos, c) in source.as_bytes().iter().enumerate() {
        let ch = *c as char;
        if ch == '.' || ch == '\n' {
            continue;
        }

        let pos = Point::from_u(pos % row_len, pos / row_len);

        if let Some(set) = antennaes.get_mut(&ch) {
            set.push(pos);
        } else {
            antennaes.insert(ch, vec![pos]);
        }
    }

    for (_ /*byte*/, positions) in antennaes.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let antinode_pos =
                    gen_positions(positions[i], positions[j], row_len, col_len, false);
                for pos in antinode_pos.iter() {
                    antinodes.insert(*pos);
                }

                // for (idx, c) in source.as_bytes().iter().enumerate() {
                //     let ch = *c as char;
                //     if ch == '\n' {
                //         print!("{}", ch);
                //         continue;
                //     }

                //     let pos = Coord::new(idx % row_len, idx / row_len);

                //     if antinode_pos.contains(&pos) {
                //         print!("{}", '#');
                //     } else if pos == positions[i] || pos == positions[j] {
                //         print!("{}", byte);
                //     } else {
                //         print!("{}", '.');
                //     }
                // }

                // println!();
            }
        }
    }

    // for (idx, c) in source.as_bytes().iter().enumerate() {
    //     let ch = *c as char;
    //     if ch != '.' {
    //         print!("{}", ch);
    //         continue;
    //     }

    //     let pos = Coord::new(idx % row_len, idx / row_len);

    //     if antinodes.contains(&pos) {
    //         print!("{}", '#');
    //     } else {
    //         print!("{}", ch);
    //     }
    // }

    antinodes.len() as u32
}

pub fn part_02(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let row_len = source.find('\n').unwrap() + 1; // deliberate, newlines will not factor into coord calcs
    let col_len = source.len() / row_len;
    let mut antennaes: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (pos, c) in source.as_bytes().iter().enumerate() {
        let ch = *c as char;
        if ch == '.' || ch == '\n' {
            continue;
        }

        let pos = Point::from_u(pos % row_len, pos / row_len);

        if let Some(set) = antennaes.get_mut(&ch) {
            set.push(pos);
        } else {
            antennaes.insert(ch, vec![pos]);
        }
    }

    for (_, positions) in antennaes.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let antinode_pos =
                    gen_positions(positions[i], positions[j], row_len, col_len, true);
                for pos in antinode_pos.iter() {
                    antinodes.insert(*pos);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn gen_positions(
    a: Point,
    b: Point,
    row_len: usize,
    col_len: usize,
    skip_check: bool,
) -> Vec<Point> {
    let mut positions = Vec::new();
    let slope = get_slope(a, b);

    fn check(pos: Point, a: Point, b: Point) -> bool {
        pos.euclid_dist(a) == pos.euclid_dist(b).scale_by(2)
            || pos.euclid_dist(b) == pos.euclid_dist(a).scale_by(2)
    }

    for pos in slope.iter(a, row_len, col_len) {
        if skip_check || check(pos, a, b) {
            positions.push(pos);
        }
    }

    for pos in slope.rev_dir().iter(a, row_len, col_len) {
        if skip_check || check(pos, a, b) {
            positions.push(pos);
        }
    }

    positions
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Slope {
    dx: isize,
    dy: isize,
}

impl Slope {
    fn rev_dir(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    fn iter(&self, start: Point, row_len: usize, col_len: usize) -> CoordIter {
        CoordIter {
            curr: start,
            slope: *self,
            row_len: row_len as isize,
            col_len: col_len as isize,
        }
    }
}

impl Add<Slope> for Point {
    type Output = Point;

    fn add(self, rhs: Slope) -> Point {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

struct CoordIter {
    slope: Slope,
    curr: Point,
    row_len: isize,
    col_len: isize,
}

impl Iterator for CoordIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr;
        if !(0..self.row_len - 1).contains(&result.x) || !(0..self.col_len).contains(&result.y) {
            return None;
        }

        self.curr = self.curr + self.slope;

        Some(result)
    }
}

fn get_slope(a: Point, b: Point) -> Slope {
    let rise = b.y - a.y;
    let run = b.x - a.x;
    let div = gcd(rise, run);
    if div == 0 {
        return Slope { dx: run, dy: rise };
    }

    Slope {
        dx: run / div,
        dy: rise / div,
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
