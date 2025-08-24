use std::collections::HashMap;

/// # Errors
/// Returns `Err` if any line does not contain exactly two numbers separated by three spaces
pub fn part_01(input: &str) -> Result<u32, String> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in input.lines() {
        let (left, right) = line
            .split_once("   ")
            .ok_or_else(|| format!("Failed to split line: {line}"))?;

        let left = left.parse::<u32>().map_err(|e| format!("Failed to parse left number: {e}"))?;
        let right = right.parse::<u32>().map_err(|e| format!("Failed to parse right number: {e}"))?;

        left_nums.push(left);
        right_nums.push(right);
    }

    left_nums.sort_unstable();
    right_nums.sort_unstable();

    // Return sum of diffs
    Ok(left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum())
}

/// # Errors
/// Returns `Err` if any line does not contain exactly two numbers separated by three spaces
pub fn part_02(input: &str) -> Result<u32, String> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in input.lines() {
        let (left, right) = line
            .split_once("   ")
            .ok_or_else(|| format!("Failed to split line: {line}"))?;

        let left = left.parse::<u32>().map_err(|e| format!("Failed to parse left number: {e}"))?;
        let right = right.parse::<u32>().map_err(|e| format!("Failed to parse right number: {e}"))?;

        left_nums.push(left);
        right_nums.push(right);
    }

    let mut left_counts = HashMap::new();
    let mut sum = 0;

    for num in &left_nums {
        if let Some(count) = left_counts.get(num) {
            sum += num * count;
        } else {
            let count = u32::try_from(right_nums.iter().filter(|&&x| x == *num).count())
                .map_err(|e| format!("Failed to convert count to u32: {e}"))?;
            left_counts.insert(num, count);

            sum += num * count;
        }
    }

    Ok(sum)
}
