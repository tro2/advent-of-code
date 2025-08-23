use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
};

pub fn part_01(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut id = 0;
    let mut strbuf = Vec::new();

    // id the file blocks
    for (idx, ch) in source.chars().enumerate() {
        let num = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            strbuf.extend(std::iter::repeat_n(Fileblock::new(id), num));
            id += 1;
        } else {
            strbuf.extend(std::iter::repeat_n(Fileblock::new(-1), num));
        }
    }

    // reorder to compress file blocks

    let mut front = strbuf.iter().position(|&c| c.id == -1).unwrap();

    let mut end = strbuf.len() - 1;
    while front < end {
        if strbuf[end].id != -1 {
            strbuf.swap(front, end);

            front += 1 + strbuf
                .iter()
                .skip(front + 1) // Start iterating from the next index
                .position(|c| c.id == -1)
                .unwrap();
        }
        end -= 1;
    }

    strbuf[0..front]
        .iter()
        .enumerate()
        .map(|(idx, fileblock)| idx * fileblock.id as usize)
        .sum()
}

pub fn part_02(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut id = 0;
    let mut files = Vec::new();
    let mut store: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    // id the file blocks
    for (idx, ch) in source.chars().enumerate() {
        let num = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            files.extend(std::iter::repeat_n(FileChunk::new(id, num, files.len()), num));
            id += 1;
        } else {
            if num == 0 {
                continue;
            }

            if let Some(set) = store.get_mut(&num) {
                set.insert(files.len());
            } else {
                store.insert(num, BTreeSet::from([files.len()]));
            }
            files.extend(std::iter::repeat_n(FileChunk::new(-1, num, files.len()), num));
        }
    }

    // reorder to compress file blocks

    let mut end = files.iter().rev().find(|c| c.id != -1).unwrap().start_idx;

    while end > 0 {
        let curr = files[end];
        // println!("id: {}", curr.id);
        // print_arr(&files);
        // print_store(&store);
        // println!();

        let max = store.last_key_value().unwrap().0;
        let idx = if curr.size <= *max {
            store
                .range(curr.size..=*max)
                .filter_map(|(_, set)| {
                    if let Some(idx) = set.first() {
                        if *idx > curr.start_idx {
                            return None;
                        }
                        Some(*idx)
                    } else {
                        None
                    }
                })
                .min()
        } else {
            None
        };

        if let Some(idx) = idx {
            let swap = FileChunk {
                start_idx: idx,
                ..curr
            };

            // split insert space into 2 chunks
            let space = files[idx];
            if space.size > curr.size {
                let new_space = FileChunk {
                    id: -1,
                    start_idx: idx + curr.size,
                    size: space.size - curr.size,
                };

                if let Some(set) = store.get_mut(&new_space.size) {
                    set.insert(new_space.start_idx);
                } else {
                    store.insert(new_space.size, BTreeSet::from([new_space.start_idx]));
                }

                let comb = std::iter::repeat_n(swap, curr.size)
                    .chain(std::iter::repeat_n(new_space, new_space.size));

                files.splice(idx..idx + curr.size + new_space.size, comb);
            } else {
                files.splice(
                    idx..idx + curr.size,
                    std::iter::repeat_n(swap, curr.size),
                );
            }

            store.get_mut(&space.size).unwrap().remove(&space.start_idx);

            // solidfy space where curr used to exist into 1 continuous empty file
            let prev = files[end - 1];
            let next = files.get(end + 1);
            let right_empty = next.copied().filter(|n| n.id == -1);

            let (end_space, to_remove) = match (prev.id == -1, right_empty) {
                (true, Some(next)) => (
                    FileChunk {
                        id: -1,
                        start_idx: prev.start_idx,
                        size: prev.size + curr.size + next.size,
                    },
                    vec![prev, next],
                ),
                (true, None) => (
                    FileChunk {
                        id: -1,
                        start_idx: prev.start_idx,
                        size: prev.size + curr.size,
                    },
                    vec![prev],
                ),
                (false, Some(next)) => (
                    FileChunk {
                        id: -1,
                        start_idx: curr.start_idx,
                        size: curr.size + next.size,
                    },
                    vec![next],
                ),
                (false, None) => (
                    FileChunk {
                        id: -1,
                        start_idx: curr.start_idx,
                        size: curr.size,
                    },
                    Vec::new(),
                ),
            };

            // update empty space map
            for chunk in to_remove {
                store.get_mut(&chunk.size).unwrap().remove(&chunk.start_idx);
            }

            if let Some(set) = store.get_mut(&end_space.size) {
                set.insert(end_space.start_idx);
            } else {
                store.insert(end_space.size, BTreeSet::from([end_space.start_idx]));
            }
            files.splice(
                end_space.start_idx..end_space.start_idx + end_space.size,
                std::iter::repeat_n(end_space, end_space.size),
            );
        }

        end = files[0..end]
            .iter()
            .rev()
            .find(|c| c.id != -1)
            .map(|c| c.start_idx)
            .unwrap_or(0);
    }

    files
        .iter()
        .enumerate()
        .map(|(idx, fileblock)| {
            if fileblock.id == -1 {
                0
            } else {
                idx * fileblock.id as usize
            }
        })
        .sum()
}

#[allow(dead_code)]
fn print_store(store: &BTreeMap<usize, BTreeSet<usize>>) {
    let mut buffer = String::new();

    buffer.push_str("keys\n");
    let keys = store
        .keys()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    buffer.push_str(&keys);
    buffer.push('\n');

    let keys: Vec<usize> = store.keys().copied().collect();
    for num in &keys {
        buffer.push_str(&format!("set {}\n", num));
        let set = store
            .get(num)
            .unwrap()
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        buffer.push_str(&set);
        buffer.push('\n');
    }
    println!("{}", buffer);
}

#[allow(dead_code)]
fn print_arr(arr: &[FileChunk]) {
    println!(
        "fileblocks\n{}",
        arr.iter()
            .map(|chunk| {
                if chunk.id == -1 {
                    ".".to_string()
                } else {
                    chunk.id.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    )
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct FileChunk {
    id: i32,
    size: usize,
    start_idx: usize,
}

impl FileChunk {
    fn new(id: i32, size: usize, start_idx: usize) -> Self {
        Self {
            id,
            size,
            start_idx,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Fileblock {
    id: i32,
}

impl Fileblock {
    fn new(id: i32) -> Self {
        Fileblock { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_creation() {
        let set1 = BTreeSet::from([1]);
        let set2: BTreeSet<_> = [1].into();

        assert_eq!(set1, set2);
    }
}
