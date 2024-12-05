use std::fs::read_to_string;

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let c_box = CharBox {
        chars: &source.as_bytes(),
        row_len: source.find("\n").unwrap()
    };

    let target = b"XMAS";
    let row = count_substring_in_iter(c_box.iter(IterDirection::Row), target);
    let row_rev = count_substring_in_iter(c_box.iter(IterDirection::Row).rev(), target);
    let col = count_substring_in_iter(c_box.iter(IterDirection::Column), target);
    let col_rev = count_substring_in_iter(c_box.iter(IterDirection::Column).rev(), target);
    let nw = count_substring_in_iter(c_box.iter(IterDirection::DiagNWtoSE), target);
    let nw_rev = count_substring_in_iter(c_box.iter(IterDirection::DiagNWtoSE).rev(), target);
    let ne = count_substring_in_iter(c_box.iter(IterDirection::DiagNEtoSW), target);
    let ne_rev = count_substring_in_iter(c_box.iter(IterDirection::DiagNEtoSW).rev(), target);

    row + row_rev + col + col_rev + nw + nw_rev + ne + ne_rev
}

fn count_substring_in_iter<I>(iter: I, target: &[u8]) -> u32
where
    I: Iterator<Item = u8>,
{
    let mut buffer = Vec::new();
    let mut count = 0;
    for byte in iter {
        buffer.push(byte);
        if buffer.len() > target.len() {
            buffer.remove(0);
        }
        if buffer == target {
            count += 1;
        }
    }
    count
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
            IterDirection::DiagNWtoSE => (0, 0),
            IterDirection::DiagNEtoSW => (self.row_len - 1, 0)
        };
        CharBoxIter {
            char_box: self,
            index: start_idx,
            col_idx,
            direction,
        }
    }
}

enum IterDirection {
    Row,
    Column,
    DiagNWtoSE,
    DiagNEtoSW,
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

    fn pos(&self) -> (isize, isize) {
        let x = self.index % self.row_len();
        let y = self.index / self.row_len();
        (x as isize, y as isize)
    }

    fn coords_to_idx(&self, x: isize, y: isize) -> Option<usize> {
        let len = self.row_len() as isize;
        if  x < 0 || x >= len || y < 0 || y >= len {
            return None;
        }
        Some(y as usize * self.row_len() + x as usize)
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
                if self.col_idx >= self.row_len() {
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
            IterDirection::DiagNWtoSE => {
                let max_cols = (self.row_len() - 1) * 2;

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
                        if self.col_idx >= self.row_len() {
                            self.coords_to_idx(
                                (self.col_idx - (self.row_len() - 1)) as isize,
                                (self.row_len() - 1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.col_idx * self.row_len()
                        };
                }

                Some(result)
            }
            IterDirection::DiagNEtoSW => {
                let max_cols = (self.row_len() - 1) * 2;

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
                        if self.col_idx >= self.row_len() {
                            self.coords_to_idx(
                                max_cols as isize - self.col_idx as isize,
                                (self.row_len() - 1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.coords_to_idx(
                                (self.row_len() - 1) as isize,
                                (self.col_idx) as isize
                            ).unwrap()
                        };
                }

                Some(result)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for CharBoxIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.direction {
            IterDirection::Row => {
                if self.index < self.char_box.chars.len() {
                    let result = self.char_box.chars[self.total_len() - 1 - self.index];
                    self.index += 1;
                    Some(result)
                } else {
                    None
                }
            }
            IterDirection::Column => {
                if self.col_idx >= self.row_len() {
                    return None;
                }
                
                let result = self.char_box.chars[self.total_len() - 1 - self.index];
                
                // Move down current column
                self.index += self.row_len();
                
                // If we hit bottom, move to top of next column
                if self.index >= self.total_len() {
                    self.col_idx += 1;
                    self.index = self.col_idx;
                }
                
                Some(result)
            }
            IterDirection::DiagNWtoSE => {
                let max_cols = (self.row_len() - 1) * 2;

                if self.col_idx > max_cols {
                    return None;
                }

                let result = self.char_box.chars[self.total_len() - 1 - self.index];

                // go up one col and right one
                let (x, y) = self.pos();
                if let Some(idx) = self.coords_to_idx(x + 1, y - 1) {
                    self.index = idx;
                } else { // if that's not valid, start at next row
                    self.col_idx += 1;
                    self.index =
                        if self.col_idx >= self.row_len() {
                            self.coords_to_idx(
                                (self.col_idx - (self.row_len() - 1)) as isize,
                                (self.row_len() - 1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.col_idx * self.row_len()
                        };
                }

                Some(result)
            }
            IterDirection::DiagNEtoSW => {
                let max_cols = (self.row_len() - 1) * 2;

                if self.col_idx > max_cols {
                    return None;
                }

                let result = self.char_box.chars[self.total_len() - 1 - self.index];

                // go up one col and left one
                let (x, y) = self.pos();
                if let Some(idx) = self.coords_to_idx(x - 1, y - 1) {
                    self.index = idx;
                } else { // if that's not valid, start at next row
                    self.col_idx += 1;
                    self.index =
                        if self.col_idx >= self.row_len() {
                            self.coords_to_idx(
                                max_cols as isize - self.col_idx as isize,
                                (self.row_len() - 1) as isize
                            ).unwrap_or(0)
                        } else {
                            self.coords_to_idx(
                                (self.row_len() - 1) as isize,
                                (self.col_idx) as isize
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
            1,2,3,4,
            5,6,7,8,
            9,10,11,12,
            13,14,15,16
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 4,
        };

        let output = b.iter(IterDirection::Row).collect::<Vec<u8>>();
        let expected = (1..=16).collect::<Vec<u8>>();

        assert_eq!(output, expected);

        let output = b.iter(IterDirection::Row).rev().collect::<Vec<u8>>();
        let expected = (1..=16).rev().collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn col_iter() {
        let chars = [
            1,5,9,13,
            2,6,10,14,
            3,7,11,15,
            4,8,12,16
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 4,
        };

        let output = b.iter(IterDirection::Column).collect::<Vec<u8>>();
        let expected = (1..=16).collect::<Vec<u8>>();

        assert_eq!(output, expected);

        let output = b.iter(IterDirection::Column).rev().collect::<Vec<u8>>();
        let expected = (1..=16).rev().collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn diag_nw_iter() {
        let chars = [
            1,3,6,10,
            2,5,9,13,
            4,8,12,15,
            7,11,14,16
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 4,
        };

        let output = b.iter(IterDirection::DiagNWtoSE).collect::<Vec<u8>>();
        let expected = (1..=16).collect::<Vec<u8>>();

        assert_eq!(output, expected);

        let output = b.iter(IterDirection::DiagNWtoSE).rev().collect::<Vec<u8>>();
        let expected = (1..=16).rev().collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

    #[test]
    fn diag_ne_iter() {
        let chars = [
            10,6,3,1,
            13,9,5,2,
            15,12,8,4,
            16,14,11,7
        ];

        let b = CharBox {
            chars: &chars,
            row_len: 4,
        };

        let output = b.iter(IterDirection::DiagNEtoSW).collect::<Vec<u8>>();
        let expected = (1..=16).collect::<Vec<u8>>();

        assert_eq!(output, expected);

        let output = b.iter(IterDirection::DiagNEtoSW).rev().collect::<Vec<u8>>();
        let expected = (1..=16).rev().collect::<Vec<u8>>();

        assert_eq!(output, expected);
    }

}
