pub fn part_01(path: &str) -> u32 {
    todo!("create different iters for possible search patterns, and call iter.find() xmas");

    0
}

#[derive(Debug)]
struct CharBox<'a> {
    chars: &'a[u8],
    row_len: usize
}

impl<'a> CharBox<'a> {
    fn new(chars: &'a [u8], row_len: usize) -> Self {
        CharBox { chars, row_len }
    }

    fn to_iter(&'a self) -> CharBoxIter<'a> {
        CharBoxIter {
            char_box: self,
            index: 0
        }
    }
}

struct CharBoxIter<'a> {
    char_box: &'a CharBox<'a>,
    index: usize
}

impl<'a> Iterator for CharBox<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
