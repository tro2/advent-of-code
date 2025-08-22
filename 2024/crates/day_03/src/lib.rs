use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part_01(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in lines.map_while(Result::ok) {
        for reg_match in reg.find_iter(&line) {
            if let Some(captures) = reg.captures(reg_match.as_str()) {
                let num1: u32 = captures[1].parse().unwrap();
                let num2: u32 = captures[2].parse().unwrap();
                sum += num1 * num2;
            }
        }
    }

    sum
}

pub fn part_02(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;
    let search_reg = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let cap_reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut perform_op = true;

    for line in lines.map_while(Result::ok) {
        for reg_match in search_reg.find_iter(&line) {
            let mat = reg_match.as_str();
            match mat {
                "do()" => {
                    println!("do");
                    perform_op = true
                }
                "don't()" => {
                    println!("don't");
                    perform_op = false
                }
                _ => {
                    if let Some(captures) = cap_reg.captures(mat) {
                        let num1: u32 = captures[1].parse().unwrap();
                        let num2: u32 = captures[2].parse().unwrap();
                        if !perform_op {
                            println!("skipped: {} * {}", num1, num2);
                            continue;
                        }
                        println!("performed: {} * {}", num1, num2);
                        sum += num1 * num2;
                    }
                }
            }
        }
    }

    sum
}
