use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut symbol_positions = Vec::new();
    
    for (line_idx, line) in lines.enumerate() {
        if let Ok(line) = line {
            for (char_idx, character) in line.chars().enumerate() {
                if (character < '0' || character > '9') && character != '.' {
                    symbol_positions.push((line_idx as i32, char_idx as i32));
                    // println!("Found symbol ({}) at ({}, {})", character, line_idx, char_idx);
                }
            }
        }
    }

    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for (line_idx, line) in lines.enumerate() {
        if let Ok(line) = line {
            let mut num = 0;
            let mut power = 0;
            let mut sym_adj = false;
            let mut num_ended = false;
            for (rev_char_idx, character) in line.chars().rev().enumerate() {
                let char_idx = line.len() - rev_char_idx - 1;
                if character >= '0' && character <= '9' {
                    if is_symbol_adj(line_idx as i32, char_idx as i32, &symbol_positions) {
                        sym_adj = true;
                    }
                    num += (character as u32 - '0' as u32) * 10u32.pow(power);
                    power += 1;
                    if char_idx == 0 {
                        num_ended = true;
                    }
                } else if num != 0 {
                    num_ended = true;
                }

                if num_ended && num != 0 {
                    if sym_adj {
                        // println!("Found sym_adj number ({}), added to sum for total of {}", num, sum+num);
                        sum += num;
                    } else {
                        // println!("Found non-sym_adj number ({}), resetting", num);
                    }
                    num = 0;
                    power = 0;
                    num_ended = false;
                    sym_adj = false;
                }
            }
        }
    }

    return sum;
}

fn is_symbol_adj(line_idx: i32, char_idx: i32, symbol_positions: &Vec<(i32, i32)>) -> bool {
    for &(y, x) in symbol_positions {
        if (y == line_idx && (x == char_idx - 1 || x == char_idx + 1)) || // left or right
           (y == line_idx - 1 && (x == char_idx - 1 || x == char_idx || x == char_idx + 1)) || // above
           (y == line_idx + 1 && (x == char_idx - 1 || x == char_idx || x == char_idx + 1)) { // below
            return true;
        }
    }
    false
}
