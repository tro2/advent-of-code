use std::{fs::read_to_string, ops::Sub};

pub fn part_01(path: &str) -> u32 {
    let mut source = read_to_string(path).unwrap()
        .lines()
        .map(|line| format!(".{}", line))
        .collect::<Vec<String>>()
        .join("\n");
    source.push('\n');
    let line_len = source.find("\n").unwrap() + 1;
    source.push_str(&".".repeat(line_len));

    let c_box = CharBox {
        chars: source.as_bytes(),
        row_len: source.find("\n").unwrap() + 1
    };

    let target = vec!['X', 'M', 'A', 'S'];

    let row = c_box.iter(IterDirection::Row).count_substring(&target);
    let col = c_box.iter(IterDirection::Column).count_substring(&target);
    let nw = c_box.iter(IterDirection::DiagSWtoNE).count_substring(&target);
    let ne = c_box.iter(IterDirection::DiagNWtoSE).count_substring(&target);
    
    let target = vec!['S', 'A', 'M', 'X'];
    
    let row_rev = c_box.iter(IterDirection::Row).count_substring(&target);
    let col_rev = c_box.iter(IterDirection::Column).count_substring(&target);
    let nw_rev = c_box.iter(IterDirection::DiagSWtoNE).count_substring(&target);
    let ne_rev = c_box.iter(IterDirection::DiagNWtoSE).count_substring(&target);

    row + row_rev + col + col_rev + nw + nw_rev + ne + ne_rev
}

pub fn part_02(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();
    let mut sum = 0;

    // iterate over all 3 by 3 blocks in the source string

    let patterns = [
        [
            "M M",
            " A ",
            "S S"
        ],
        [
            "S S",
            " A ",
            "M M"
        ],
        [
            "M S",
            " A ",
            "M S"
        ],
        [
            "S M",
            " A ",
            "S M"
        ]
    ];


    let lines = source
        .lines()
        .collect::<Vec<&str>>();

    let line_groups = lines.windows(3);

    for group in line_groups {
        let mut blocks = Vec::new();

        for line in group {
            let line_windows = line.as_bytes().windows(3);
            for (idx, window) in line_windows.enumerate() {
                if blocks.len() <= idx {
                    blocks.push(Vec::new())
                }
                // convert window to str
                blocks[idx].push(std::str::from_utf8(window).unwrap());
            }
        }

        sum += blocks.iter()
            .filter(|block| {
                patterns.iter().any(|pattern| compare(block, pattern))
            })
            .count()
    }

    sum as u32
}

fn compare(block: &[&str], pattern: &[&str; 3]) -> bool {
    block.iter().zip(pattern.iter()).all(|(b, p)| {
        b.bytes().zip(p.bytes()).all(|(b_byte, p_byte)| {
            p_byte == b' ' || b_byte == p_byte
        })
    })
}

#[derive(Debug)]
struct CharBox<'a> {
    chars: &'a[u8],
    row_len: usize
}

impl<'a> CharBox<'a> {
    fn iter(&'a self, direction: IterDirection) -> CharBoxIter<'a> {
        let (start_idx, col_idx) = match direction {
            IterDirection::Row => (0, 0),
            IterDirection::Column => (0, 0),
            IterDirection::DiagSWtoNE => (0, 0),
            IterDirection::DiagNWtoSE => (self.row_len - 1, 0)
        };
        CharBoxIter {
            char_box: self,
            index: start_idx,
            col_idx,
            direction,
        }
    }
}

#[derive(Debug)]
enum IterDirection {
    Row,
    Column,
    DiagSWtoNE,
    DiagNWtoSE,
}

struct CharBoxIter<'a> {
    char_box: &'a CharBox<'a>,
    index: usize,
    col_idx: usize,
    direction: IterDirection,
}

impl<'a> CharBoxIter<'a> {
    fn total_len(&self) -> usize {
        self.char_box.chars.len()
    }

    fn row_len(&self) -> usize {
        self.char_box.row_len
    }

    fn col_len(&self) -> usize {
        self.total_len() / self.row_len()
    }

    fn pos(&self) -> (isize, isize) {
        let x = self.index % self.row_len();
        let y = self.index / self.row_len();
        (x as isize, y as isize)
    }

    fn coords_to_idx(&self, x: isize, y: isize) -> Option<usize> {
        let row_len = self.row_len() as isize;
        let col_len = self.col_len() as isize;
        if  x < 0 || x >= row_len || y < 0 || y >= col_len {
            return None;
        }
        Some((y * row_len + x) as usize)
    }

    fn count_substring(&mut self, target: &[char]) -> u32 {
        let mut buffer = Vec::new();
        let mut count = 0;

        // let mut idx_buf = Vec::new();
        // let mut idx_coords = Vec::new();
        // println!("num cols {} num rows {}", self.row_len(), self.col_len());
        // let mut array: Vec<Vec<char>> = vec![vec!['.'; self.row_len()]; self.col_len()];
        // idx_buf.push(self.index);
        // idx_coords.push(self.pos());
        while let Some(byte) = self.next() {
            // idx_buf.push(self.index);
            // idx_coords.push(self.pos());
            buffer.push(byte as char);
            if buffer.len() > target.len() {
                // idx_buf.remove(0);
                // idx_coords.remove(0);
                buffer.remove(0);
            }
            if buffer == target {
                // array[idx_coords[0].1 as usize][idx_coords[0].0 as usize] = target[0];
                // array[idx_coords[1].1 as usize][idx_coords[1].0 as usize] = target[1];
                // array[idx_coords[2].1 as usize][idx_coords[2].0 as usize] = target[2];
                // array[idx_coords[3].1 as usize][idx_coords[3].0 as usize] = target[3];
                count += 1;
            }
        }
        // println!("{:?} {:?}", self.direction, target);
        // for row in array {
        //     println!("{:?}", row);
        // }
        count
    }
}

impl<'a> Iterator for CharBoxIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            IterDirection::Row => {
                if self.index < self.char_box.chars.len() {
                    let result = self.char_box.chars[self.index];
                    self.index += 1;
                    Some(result)
                } else {
                    None
                }
            }
            IterDirection::Column => {
                if self.col_idx > self.col_len() {
                    return None;
                }
                
                let result = self.char_box.chars[self.index];
                
                // Move down current column
                self.index += self.row_len();
                
                // If we hit bottom, move to top of next column
                if self.index >= self.total_len() {
                    self.col_idx += 1;
                    self.index = self.col_idx;
                }
                
                Some(result)
            }
            IterDirection::DiagSWtoNE => {
                let max_cols = self.col_len() + self.row_len() - 2;

                if self.col_idx > max_cols {
                    return None;
                }

                let result = self.char_box.chars[self.index];

                // go up one col and right one
                let (x, y) = self.pos();
                if let Some(idx) = self.coords_to_idx(x + 1, y - 1) {
                    self.index = idx;
                } else { // if that's not valid, start at next row
                    self.col_idx += 1;
                    self.index =
                        if self.col_idx >= self.col_len() {
                            self.coords_to_idx(
                                self.col_idx.sub(self.col_len() - 1) as isize,
                                self.col_len().sub(1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.coords_to_idx(
                                0,
                                self.col_idx as isize
                            ).unwrap_or(0)
                        };
                }

                Some(result)
            }
            IterDirection::DiagNWtoSE => {
                let max_cols = self.col_len() + self.row_len() - 2;

                if self.col_idx > max_cols {
                    return None;
                }

                let result = self.char_box.chars[self.index];

                // go up one col and left one
                let (x, y) = self.pos();
                if let Some(idx) = self.coords_to_idx(x - 1, y - 1) {
                    self.index = idx;
                } else { // if that's not valid, start at next row
                    self.col_idx += 1;
                    self.index =
                        if self.col_idx >= self.col_len() {
                            self.coords_to_idx(
                                max_cols as isize - self.col_idx as isize,
                                self.col_len().sub(1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.coords_to_idx(
                                self.row_len().sub(1) as isize,
                                self.col_idx as isize
                            ).unwrap()
                        };
                }

                Some(result)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_iter() {
        let chars = [
             1, 2, 3, 4, 5,
             6, 7, 8, 9,10,
            11,12,13,14,15,
            16,17,18,19,20
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 5,
        };

        let output = b.iter(IterDirection::Row).collect::<Vec<u8>>();
        let expected = (1..=20).collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn col_iter() {
        let chars = [
            1, 5, 9,13,17,
            2, 6,10,14,18,
            3, 7,11,15,19,
            4, 8,12,16,20
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 5,
        };

        let output = b.iter(IterDirection::Column).collect::<Vec<u8>>();
        let expected = (1..=20).collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }
    
    #[test]
    fn diag_nw_iter() {
        let chars = [
            1, 3, 6,10,14,
            2, 5, 9,13,17,
            4, 8,12,16,19,
            7,11,15,18,20
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 5,
        };

        let output = b.iter(IterDirection::DiagSWtoNE).collect::<Vec<u8>>();
        let expected = (1..=20).collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn diag_ne_iter() {
        let chars = [
            14,10, 6, 3, 1,
            17,13, 9, 5, 2,
            19,16,12, 8, 4,
            20,18,15,11, 7
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 5,
        };

        let output = b.iter(IterDirection::DiagNWtoSE).collect::<Vec<u8>>();
        let expected = (1..=20).collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

}
