use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in lines.map_while(Result::ok) {
        let tens = line.chars().find(|c|c.is_ascii_digit()).unwrap();
        let ones = line.chars().rfind(|c|c.is_ascii_digit()).unwrap();
        sum = sum + tens.to_digit(10).unwrap() * 10 + ones.to_digit(10).unwrap();
    }

    sum
}

pub fn parse_num(s: &str) -> Option<u32> {
    if s.starts_with('1') || s.starts_with("one") {
        return Some(1);
    } else if s.starts_with('2') || s.starts_with("two") {
        return Some(2);
    } else if s.starts_with('3') || s.starts_with("three") {
        return Some(3);
    } else if s.starts_with('4') || s.starts_with("four") {
        return Some(4);
    } else if s.starts_with('5') || s.starts_with("five") {
        return Some(5);
    } else if s.starts_with('6') || s.starts_with("six") {
        return Some(6);
    } else if s.starts_with('7') || s.starts_with("seven") {
        return Some(7);
    } else if s.starts_with('8') || s.starts_with("eight") {
        return Some(8);
    } else if s.starts_with('9') || s.starts_with("nine") {
        return Some(9);
    }
    None
}

pub fn part_2(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in lines.map_while(Result::ok) {
        // Iterate forward for first digit (10's place)
        for i in 0..line.len() {
            if let Some(num) = parse_num(&line[i..]) {
                sum += num * 10;
                break;
            }
        }

        // Iterate backward for last digit (1's place)
        for i in (0..line.len()).rev() {
            if let Some(num) = parse_num(&line[i..]) {
                sum += num;
                break;
            }
        }
    }

    sum
}
