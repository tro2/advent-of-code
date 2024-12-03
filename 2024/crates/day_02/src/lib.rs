use std::{fs::File, io::{BufRead, BufReader}};

pub fn part_01(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();

    lines.map_while(Result::ok).filter(|line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let low_variance = nums.windows(2)
            .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));

        if !low_variance {
            return false;
        }

        let increasing = nums.windows(2)
            .all(|w| w[0] < w[1]);
        let decreasing = nums.windows(2)
            .all(|w| w[0] > w[1]);

        increasing || decreasing
    }).count() as u32
}

pub fn part_02(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();

    let is_valid = |nums: &[u32]| {
        let low_variance = nums.windows(2)
            .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));
        let increasing = nums.windows(2)
            .all(|w| w[0] < w[1]);
        let decreasing = nums.windows(2)
            .all(|w| w[0] > w[1]);
        low_variance && (increasing || decreasing)
    };

    lines.map_while(Result::ok).filter(|line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_valid(&nums) {
            return true;
        }

        (0..nums.len()).any(|i| {
            let mut temp = nums.clone();
            temp.remove(i);
            is_valid(&temp)
        })
    }).count() as u32
}