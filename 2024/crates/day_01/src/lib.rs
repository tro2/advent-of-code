use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in lines.map_while(Result::ok) {
        let (left, right) = line
            .split_once("   ")
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unwrap();
        left_nums.push(left);
        right_nums.push(right);
    }

    left_nums.sort();
    right_nums.sort();

    // return sum of diffs
    left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum()
}

pub fn part_02(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in lines.map_while(Result::ok) {
        let (left, right) = line
            .split_once("   ")
            .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
            .unwrap();
        left_nums.push(left);
        right_nums.push(right);
    }

    let mut left_counts = HashMap::new();
    let mut sum = 0;

    for num in left_nums.iter() {
        if left_counts.contains_key(num) {
            sum += num * left_counts.get(num).unwrap();
        } else {
            let count = right_nums.iter().filter(|&&x| x == *num).count() as u32;
            left_counts.insert(num, count);

            sum += num * count;
        }
    }

    sum
}
