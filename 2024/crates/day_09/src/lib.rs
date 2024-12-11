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
            strbuf.swap(front,end);

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
            files.extend(std::iter::repeat(FileChunk::new(-1, num, files.len())).take(num));
        }
    }

    // reorder to compress file blocks

    let mut end = files.iter().rev()
        .find(|c| c.id != -1)
        .unwrap()
        .start_idx;

    while end > 0 {
        let curr = files[end];

        let idx = files.iter()
            .find(|&&chunk| chunk.id == -1 && chunk.size >= curr.size && chunk.start_idx < curr.start_idx)
            .map(|res| res.start_idx);

        if let Some(idx) = idx {
            let swap = FileChunk {
                start_idx:idx,
                ..curr
            };

            // split insert space into 2 chunks
            let space = files[idx];
            if space.size > curr.size {
                let new_space = FileChunk {
                    id: -1,
                    start_idx: idx + curr.size,
                    size: space.size - curr.size
                };

                let comb = std::iter::repeat(swap).take(curr.size)
                    .chain(std::iter::repeat(new_space).take(new_space.size));

                files.splice(idx..idx + curr.size + new_space.size, comb);
            } else {
                files.splice(idx..idx + curr.size, std::iter::repeat(swap).take(curr.size));
            }
            
            // solidfy space where curr used to exist into 1 continuous empty file
            let end_space = if files[end - 1].id == -1 {
                if end < files.len() - 1 && files[end + 1].id == -1 {
                    FileChunk {
                        id: -1,
                        start_idx: files[end - 1].start_idx,
                        size: files[end - 1].size + curr.size + files[end + 1].size
                    }
                } else {
                    FileChunk {
                        id: -1,
                        start_idx: files[end - 1].start_idx,
                        size: files[end - 1].size + curr.size
                    }
                }
            } else if end < files.len() - 1 && files[end + 1].id == -1 {
                FileChunk {
                        id: -1,
                        start_idx: curr.start_idx,
                        size: curr.size + files[end + 1].size
                    }
            } else {
                FileChunk {
                    id: -1,
                    start_idx: curr.start_idx,
                    size: curr.size
                }
            };

            files.splice(end_space.start_idx..end_space.start_idx + end_space.size, std::iter::repeat(end_space).take(end_space.size));
        }

        end = files[0..end].iter().rev()
            .find(|c| c.id != -1)
            .map(|c| c.start_idx)
            .unwrap_or(0);
    }

    files.iter().enumerate()
        .map(|(idx, fileblock)| {
            if fileblock.id == -1 {
                0
            } else {
                idx * fileblock.id as usize
            }
        })
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
