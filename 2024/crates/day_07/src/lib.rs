use std::fs::read_to_string;

pub fn part_01(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut sum = 0;

    for line in source.lines() {
        let (answer, nums) = line.split_once(':').unwrap();
        let answer: usize = answer.parse().unwrap();
        let nums: Vec<usize> = nums
            .split_whitespace()
            .rev()
            .map(|num| num.parse().unwrap())
            .collect();

        if is_valid(answer, &nums) {
            sum += answer;
        }
    }

    return sum;

    fn is_valid(answer: usize, nums: &Vec<usize>) -> bool {
        if nums.len() == 1 {
            return answer == nums[0];
        }

        let mut add = nums.to_owned();
        let sum = add.pop().unwrap() + add.pop().unwrap();
        add.push(sum);

        let mut mult = nums.to_owned();
        let product = mult.pop().unwrap() * mult.pop().unwrap();
        mult.push(product);

        if sum <= answer && is_valid(answer, &add) || product <= answer && is_valid(answer, &mult) {
            return true;
        }
        false
    }
}

pub fn part_02(path: &str) -> usize {
    let source = read_to_string(path).unwrap();
    let mut sum = 0;

    for line in source.lines() {
        let (answer, nums) = line.split_once(':').unwrap();
        let answer: usize = answer.parse().unwrap();
        let nums: Vec<usize> = nums
            .split_whitespace()
            .rev()
            .map(|num| num.parse().unwrap())
            .collect();

        if is_valid(answer, &nums) {
            sum += answer;
        }
    }

    return sum;

    fn is_valid(answer: usize, nums: &Vec<usize>) -> bool {
        if nums.len() == 1 {
            return answer == nums[0];
        }

        let mut add = nums.to_owned();
        let sum = add.pop().unwrap() + add.pop().unwrap();
        add.push(sum);

        let mut mult = nums.to_owned();
        let product = mult.pop().unwrap() * mult.pop().unwrap();
        mult.push(product);

        let mut comb = nums.to_owned();
        let a = comb.pop().unwrap();
        let b = comb.pop().unwrap();
        let combination: usize = format!("{}{}", a, b).parse().unwrap();
        comb.push(combination);

        if sum <= answer && is_valid(answer, &add)
            || product <= answer && is_valid(answer, &mult)
            || combination <= answer && is_valid(answer, &comb)
        {
            return true;
        }
        false
    }
}
