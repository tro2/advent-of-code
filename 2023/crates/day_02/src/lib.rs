use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_01(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in lines.map_while(Result::ok) {
        let (game, content) = line.split_once(":").unwrap();
        let game_id = game.split_whitespace().last().unwrap().parse::<u32>().unwrap();
        let mut valid = true;

        for pull in content[1..].split(&[',', ';']) {
            let (count, color) = pull.trim().split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();
            match color {
                "red" => {
                    if count > 12 {
                        valid = false;
                        break;
                    }
                },
                "green" => {
                    if count > 13 {
                        valid = false;
                        break;
                    }
                },
                "blue" => {
                    if count > 14 {
                        valid = false;
                        break;
                    }
                },
                _ => panic!()
            }
        }
        if valid {
            sum += game_id;
        }
    }

    sum
}

pub fn part_02(path: &str) -> u32 {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut sum = 0;

    for line in lines.map_while(Result::ok) {
        let content = &line[line.find(':').unwrap() + 1..];
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for pull in content.split(&[',', ';']) {
            let (count, color) = pull.trim().split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();
            match color {
                "red" => {
                    if count > max_red {
                        max_red = count;
                    }
                },
                "green" => {
                    if count > max_green {
                        max_green = count;
                    }
                },
                "blue" => {
                    if count > max_blue {
                        max_blue = count;
                    }
                },
                _ => panic!()
            }
        }

        sum += max_red * max_green * max_blue;
    }

    sum
}
