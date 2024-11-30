use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let source = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in source.map_while(Result::ok) {
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

pub fn part_02(path: &str) -> u32 {
    let source = BufReader::new(File::open(path).unwrap()).lines();
    let mut card_counts = HashMap::<u32, u32>::new();

    for line in source.map_while(Result::ok) {
        // Line format
        // Game 1: winningnum1 winningnum2 winningnum3 ... | num1 num2 ...
        let (id, game) = line.split_once(":").unwrap();
        
        // id: "Card 1383"
        let game_id = id.split_whitespace().last().unwrap().parse::<u32>().unwrap();

        // determine count of winning nums on card
        let (win_nums, found_nums) = game.split_once("|").unwrap();
        let winning_nums = extract_nums(&win_nums[1..]);
        let found_nums = extract_nums(&found_nums[1..]);
        let win_count = get_win_count(&winning_nums, &found_nums);

        // update card counts
        let num_copies = card_counts.get(&game_id).copied().unwrap_or(0) + 1;
        card_counts.insert(game_id, num_copies);
        for i in game_id + 1..game_id + win_count + 1 {
            let new_count = num_copies + card_counts.get(&i).copied().unwrap_or(0);
            card_counts.insert(i, new_count);
        }
    }

    card_counts.values().sum()
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