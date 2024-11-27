use std::{char, fs::read_to_string};
use shared::{Arrayto2DGrid, Offsets};

struct SchematicString<'a> {
    bytes: &'a [u8],
    row_len: usize,
}

impl<'a> SchematicString<'a> {
    fn new(bytes: &'a [u8], row_len: usize) -> Self {
        Self { bytes, row_len }
    }
}

impl Arrayto2DGrid for SchematicString<'_> {
    fn array_len(&self) -> usize {
        self.bytes.len()
    }

    fn row_len(&self) -> usize {
        self.row_len
    }
}

fn is_symbol(byte: u8) -> bool {
    (byte < b'0' || byte > b'9') && byte != b'.' && byte != b'\n'
}

fn check_neighbors_for_symbol(idx: usize, schematic: &SchematicString) -> bool {
    Offsets::ALL.iter().any(|offset| {
        if let Some(t_idx) = schematic.translate_idx(idx, offset) {
            is_symbol(schematic.bytes[t_idx])
        } else {
            false
        }
    })
}

// given an index that is a number, extract and parse that number
fn parse_num(idx: usize, schematic: &SchematicString) -> u32 {
    let char = schematic.bytes[idx] as char;
    if char.to_digit(10) == None {
        panic!("parse_num called on non-number");
    }

    // in a valid number
    let startidx = (0..idx).rev()
    .find(|&i| (schematic.bytes[i] as char).to_digit(10).is_none())
    .map_or(0, |i| i + 1);

    let endidx = (idx..schematic.bytes.len())
    .find(|&i| (schematic.bytes[i] as char).to_digit(10).is_none())
    .unwrap_or(schematic.array_len());

    std::str::from_utf8(&schematic.bytes[startidx..endidx])
        .unwrap()
        .parse::<u32>()
        .ok()
        .unwrap()
}

pub fn part_01(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    let schematic = SchematicString::new(input.as_bytes(), input.find('\n').unwrap() + 1);

    let mut sum = 0;
    let mut curr_num = 0;
    let mut found_sym = false;

    for (idx, byte) in schematic.bytes.iter().enumerate() {
        let c = *byte as char;
        if let Some(digit) = c.to_digit(10) {
            curr_num = curr_num * 10 + digit;
            if !found_sym {
                found_sym = check_neighbors_for_symbol(idx, &schematic);
            }
        } else {
            // at end of number
            if found_sym {
                sum += curr_num;
                found_sym = false;
            }
            curr_num = 0;
        }
    }

    return sum;
}

pub fn part_02(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    let schematic = SchematicString::new(input.as_bytes(), input.find('\n').unwrap() + 1);

    let mut sum = 0;

    for (idx, byte) in schematic.bytes.iter().enumerate() {
        if byte != &b'*' {
            continue;
        }

        let nums = Offsets::ALL.iter()
            .filter_map(|offset| {
                if let Some(t_idx) = schematic.translate_idx(idx, offset) {
                    if schematic.bytes[t_idx] >= b'0' && schematic.bytes[t_idx] <= b'9' {
                        return Some(parse_num(t_idx, &schematic));
                    }
                }
                None
            }).collect::<Vec<_>>();
        
        // filter list to unique nums and if there are 2, multiply them together and add to sum
        let unique_nums = nums.iter().cloned().collect::<std::collections::HashSet<_>>();
        if unique_nums.len() == 2 {
            println!("{:?}", unique_nums);
            sum += unique_nums.iter().product::<u32>();
        }
    }

    sum
}
