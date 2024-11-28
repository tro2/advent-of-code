use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let source = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in source.filter_map(Result::ok) {
        // Line format
        // Game 1: winningnum1 winningnum2 winningnum3 ... | num1 num2 ...
        let (_id, game) = line.split_once(":").unwrap();
        let (win_nums, found_nums) = game.split_once("|").unwrap();

        let winning_nums = extract_nums(&win_nums[1..]);
        let found_nums = extract_nums(&found_nums[1..]);
        let count = get_win_count(&winning_nums, &found_nums);

        if count > 0 {
            sum += 2_u32.pow(count-1);
        }
    }
    
    sum
}

// takes a string with space separated unsigned numbers and returns them in a vector
// Example Input: " 83 273 93 10 28 1 82 "
fn extract_nums(source: &str) -> Vec<u32> {
    source.split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn get_win_count(win_nums: &[u32], found_nums: &[u32]) -> u32 {
    win_nums.iter().filter(|&num| found_nums.contains(num)).count() as u32
}