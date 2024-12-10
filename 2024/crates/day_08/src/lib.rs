use std::{collections::{HashMap, HashSet}, fs::read_to_string, ops::Add};

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let row_len = source.find('\n').unwrap() + 1; // deliberate, newlines will not factor into coord calcs
    let col_len = source.len() / row_len;
    let mut antennaes = HashMap::new();
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (pos, c) in source.as_bytes().iter().enumerate() {
        let ch = *c as char;
        if ch == '.' || ch == '\n' {
            continue;
        }

        let pos = Coord::new(pos % row_len, pos / row_len);

        if antennaes.contains_key(&ch) {
            let set: &mut Vec<Coord> = antennaes.get_mut(&ch).unwrap();
            set.push(pos);
        } else {
            let mut set = Vec::new();
            set.push(pos);
            antennaes.insert(ch, set);
        }
    }

    for (_/*byte*/, positions) in antennaes.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let antinode_pos = gen_positions(positions[i], positions[j], row_len, col_len);
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
    let mut antennaes = HashMap::new();
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (pos, c) in source.as_bytes().iter().enumerate() {
        let ch = *c as char;
        if ch == '.' || ch == '\n' {
            continue;
        }

        let pos = Coord::new(pos % row_len, pos / row_len);

        if antennaes.contains_key(&ch) {
            let set: &mut Vec<Coord> = antennaes.get_mut(&ch).unwrap();
            set.push(pos);
        } else {
            let mut set = Vec::new();
            set.push(pos);
            antennaes.insert(ch, set);
        }
    }

    for (_, positions) in antennaes.iter() {
        for i in 0..positions.len() - 1 {
            for j in i + 1..positions.len() {
                let antinode_pos = gen_positions_2(positions[i], positions[j], row_len, col_len);
                for pos in antinode_pos.iter() {
                    antinodes.insert(*pos);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn gen_positions(a: Coord, b: Coord, row_len: usize, col_len: usize) -> Vec<Coord> {
    let mut positions = Vec::new();
    let slope = a.get_slope(b);

    for pos in a.iter(slope, row_len, col_len) {
        if pos.euclid_dist(a) == pos.euclid_dist(b).scale_by(2) ||
           pos.euclid_dist(b) == pos.euclid_dist(a).scale_by(2) {
            positions.push(pos);
        }

        
    }

    for pos in a.iter_rev(slope, row_len, col_len) {
        if pos.euclid_dist(a) == pos.euclid_dist(b).scale_by(2) ||
           pos.euclid_dist(b) == pos.euclid_dist(a).scale_by(2) {
            positions.push(pos);
        }
    }

    positions
}

fn gen_positions_2(a: Coord, b: Coord, row_len: usize, col_len: usize) -> HashSet<Coord> {
    let mut positions = HashSet::new();
    let slope = a.get_slope(b);

    for pos in a.iter(slope, row_len, col_len) {
        positions.insert(pos);
    }

    for pos in a.iter_rev(slope, row_len, col_len) {
        positions.insert(pos);
    }

    positions
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize
        }
    }

    fn get_slope(&self, b: Self) -> Self {
        let rise = b.y - self.y;
        let run = b.x - self.x;
        let div = gcd(rise, run);
        if div == 0 {
            return Self {
                x: run,
                y: rise
            }
        }

        Self {
            x: run / div,
            y: rise / div
        }
    }

    fn rev_dir(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y
        }
    }

    fn euclid_dist(&self, b: Self) -> Self {
        Self {
            x: (b.x - self.x).abs(),
            y: (b.y - self.y).abs()
        }
    }

    fn scale_by(&self, factor: isize) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor
        }
    }

    fn iter(&self, slope: Self, row_len: usize, col_len: usize) -> CoordIter {
        CoordIter {
            curr: *self,
            slope,
            row_len: row_len as isize,
            col_len: col_len as isize
        }
    }

    fn iter_rev(&self, slope: Self, row_len: usize, col_len: usize) -> CoordIter {
        CoordIter {
            curr: *self,
            slope: slope.rev_dir(),
            row_len: row_len as isize,
            col_len: col_len as isize
        }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct CoordIter {
    slope: Coord,
    curr: Coord,
    row_len: isize,
    col_len: isize
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.curr;
        if !(0..self.row_len - 1).contains(&result.x)
        || !(0..self.col_len).contains(&result.y) {
            return None;
        }

        self.curr = self.curr + self.slope;

        return Some(result);
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
