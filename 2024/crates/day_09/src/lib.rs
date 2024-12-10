use std::fs::read_to_string;

pub fn part_01(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut id = 0;
    let mut strbuf = Vec::new();

    // id the file blocks
    for (idx, ch) in source.chars().enumerate() {
        let num = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            strbuf.extend(std::iter::repeat(Fileblock::new(id)).take(num));
            id += 1;
        } else {
            strbuf.extend(std::iter::repeat(Fileblock::new(-1)).take(num));
        }
    }

    // reorder to compress file blocks

    let mut front = strbuf.iter()
        .position(|&c| c.id == -1)
        .unwrap();

    let mut end = strbuf.len() - 1;
    while front < end {
        if strbuf[end].id != -1 {
            let tmp = strbuf[front];
            strbuf[front] = strbuf[end];
            strbuf[end] = tmp;

            front += 1 + strbuf.iter()
                .skip(front + 1) // Start iterating from the next index
                .position(|c| c.id == -1)
                .unwrap();
        }
        end -= 1;
    }

    strbuf[0..front].iter().enumerate()
        .map(|(idx, fileblock)| idx * fileblock.id as usize)
        .sum()
}

pub fn part_02(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut id = 0;
    let mut files = Vec::new();

    // id the file blocks
    for (idx, ch) in source.chars().enumerate() {
        let num = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            files.extend(std::iter::repeat(FileChunk::new(id, num, files.len())).take(num));
            id += 1;
        } else {
            files.extend(std::iter::repeat(FileChunk::new(id, num, files.len())).take(num));
        }
    }

    // reorder to compress file blocks

    let mut front = files.iter()
        .position(|&c| c.id == -1)
        .unwrap();

    let mut end = files.iter().rev()
        .position(|&c| c.id == -1)
        .unwrap() + 1;

    while end > 0 {
        let curr = files[end];

        let idx = files.iter()
            .find(|&&chunk| chunk.id == -1 && chunk.size >= curr.size && chunk.start_idx < curr.start_idx)
            .map(|res| res.start_idx);

        if let Some(idx) = idx {
            // split insert space into 2 chunks
            
            // solidfy space where curr used to exist into 1 continuous empty file
        }

        end = files.iter().rev()
            .skip(files.len() - end)
            .find(|c| c.id != -1)
            .unwrap()
            .start_idx;
    }

    files[0..front].iter().enumerate()
        .map(|(idx, fileblock)| idx * fileblock.id as usize)
        .sum()
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct FileChunk {
    id: i32,
    size: usize,
    start_idx: usize
}

impl FileChunk {
    fn new(id: i32, size: usize, start_idx: usize) -> Self {
        Self {
            id,
            size,
            start_idx
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Fileblock {
    id: i32
}

impl Fileblock {
    fn new(id: i32) -> Self {
        Fileblock {
            id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
