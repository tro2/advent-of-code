use std::{collections::HashMap, fs::read_to_string};

pub fn part_01(path: &str, depth: u32) -> usize {
    let source = read_to_string(path).unwrap();

    let mut nums: Vec<usize> = source
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    std::iter::repeat_n(Vec::<usize>::new(), depth as usize)
        .for_each(|mut arr| {
            for num in &nums {
                let num = *num;
                let num_str = num.to_string();
                if num == 0 {
                    arr.push(1);
                } else if num_str.len() % 2 == 0 {
                    let (first, last) = num_str.split_at(num_str.len() / 2);
                    arr.push(first.parse().unwrap());
                    arr.push(last.parse().unwrap());
                } else {
                    arr.push(num * 2024);
                }
            }

            nums = arr;
        });

    nums.len()
}

type Level = u32;
type Count = usize;
type Number = usize;

pub fn part_02(path: &str, depth: u32) -> usize {
    let source = read_to_string(path).unwrap();
    let mut map: HashMap<(Number, Level), Count> = HashMap::new();

    let nums: Vec<usize> = source
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let res = nums
        .iter()
        .map(|num| calc_size(*num, 1, depth, &mut map))
        .sum();

    res
}

fn calc_size(
    num: Number,
    level: Level,
    cap: Level,
    map: &mut HashMap<(Number, Level), Count>,
) -> Count {
    if level == cap {
        if num == 0 {
            return 1;
        } else if num.to_string().len() % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    }

    if let Some(count) = map.get(&(num, level)) {
        *count
    } else {
        let num_str = num.to_string();
        let count = if num == 0 {
            calc_size(1, level + 1, cap, map)
        } else if num_str.len() % 2 == 0 {
            let (a, b) = num_str.split_at(num_str.len() / 2);
            calc_size(a.parse().unwrap(), level + 1, cap, map)
                + calc_size(b.parse().unwrap(), level + 1, cap, map)
        } else {
            calc_size(num * 2024, level + 1, cap, map)
        };
        map.insert((num, level), count);
        count
    }
}

#[allow(dead_code)]
fn display(arr: &[usize]) {
    let display: Vec<String> = arr.iter().map(|num| num.to_string()).collect();
    println!("{}", display.join(" "));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let source = "0 1 10 99 999";

        let mut map: HashMap<(Number, Level), Count> = HashMap::new();

        let nums: Vec<usize> = source
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let sum: usize = nums.iter().map(|num| calc_size(*num, 1, 1, &mut map)).sum();

        assert_eq!(sum, 7)
    }

    #[test]
    fn small() {
        let source = "125 17";

        let mut map: HashMap<(Number, Level), Count> = HashMap::new();

        let nums: Vec<usize> = source
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let sum: usize = nums.iter().map(|num| calc_size(*num, 1, 6, &mut map)).sum();

        assert_eq!(sum, 22)
    }
}
