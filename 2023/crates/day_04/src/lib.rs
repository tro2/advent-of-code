use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let source = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in source {
        if line.is_err() {
            break;
        }
        let line = line.unwrap();

        // Line format
        // Game 1: winningnum1 winningnum2 winningnum3 ... | num1 num2 ...
        let (_id, game) = line.split_at(line.find(":").unwrap());
        let (winning_nums, nums) = game.split_at(game.find("|").unwrap());
        let win_nums = extract_nums(&winning_nums[1..]);
        let found_nums = extract_nums(&nums[1..]);
        let count = win_nums.iter().filter(|num| found_nums.contains(num)).count() as u32;

        if count > 0 {
            sum += 2_u32.pow(count-1);
        }
    }
    
    sum
}

fn extract_nums(source: &str) -> Vec<u32> {
    source.trim().split(' ')
        .filter(|p| !p.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}