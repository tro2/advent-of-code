use regex::Regex;

/// # Errors
///
/// Will return Err if the regex string is invalid or the input is incorrectly formatted
pub fn part_01(input: &str) -> Result<u32, String> {
    let mut sum = 0;

    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .map_err(|e| format!("Invalid regex detected: {e}"))?;

    for line in input.lines() {
        for reg_match in reg.find_iter(line) {
            if let Some(captures) = reg.captures(reg_match.as_str()) {
                let num1: u32 = captures[1]
                    .parse()
                    .map_err(|e| format!("Failed to parse first operand: {e}"))?;
                let num2: u32 = captures[2]
                    .parse()
                    .map_err(|e| format!("Failed to parse second operand: {e}"))?;

                sum += num1 * num2;
            }
        }
    }

    Ok(sum)
}

/// # Errors
///
/// Will return Err if the regex string is invalid or the input is incorrectly formatted
pub fn part_02(input: &str) -> Result<u32, String> {
    let mut sum = 0;

    let search_reg = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)")
        .map_err(|e| format!("Invalid search regex: {e}"))?;
    let cap_reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .map_err(|e| format!("Invalid capture regex: {e}"))?;

    let mut perform_op = true;

    for line in input.lines() {
        for reg_match in search_reg.find_iter(&line) {
            let mat = reg_match.as_str();
            match mat {
                "do()" => {
                    println!("do");
                    perform_op = true;
                }
                "don't()" => {
                    println!("don't");
                    perform_op = false;
                }
                _ => {
                    if let Some(captures) = cap_reg.captures(mat) {
                        let num1: u32 = captures[1]
                            .parse()
                            .map_err(|e| format!("Failed to parse first operand: {e}"))?;
                        let num2: u32 = captures[2]
                            .parse()
                            .map_err(|e| format!("Failed to parse second operand: {e}"))?;

                        if !perform_op {
                            println!("skipped: {num1} * {num2}");
                            continue;
                        }
                        println!("performed: {num1} * {num2}");

                        sum += num1 * num2;
                    }
                }
            }
        }
    }

    Ok(sum)
}
