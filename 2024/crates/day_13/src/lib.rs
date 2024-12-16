use std::fs::read_to_string;

use nalgebra::{Matrix2, Vector2};

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();
    let mut tokens = 0;

    for group in source.split("\n\n") {
        let data: Vec<(f64, f64)> = group.lines()
            .map(|line| {
                let (_, numstr) = line.split_once(':').unwrap();
                let (a, b) = numstr.split_once(',').unwrap();
                (a[3..].parse().unwrap(), b[3..].parse().unwrap())
            })
            .collect();
        
        let (a, b, prize) = (data[0], data[1], data[2]);
        
        let coefficients = Matrix2::new(
            a.0, b.0,
            a.1, b.1
        );

        let end = Vector2::new(
            prize.0,
            prize.1
        );
        let decomp = coefficients.lu();
        if let Some(x) = decomp.solve(&end) {
            if let Some(count) = token_count(coefficients, end, x) {
                tokens += count;
            }
        }

    }

    tokens
}

pub fn part_02(path: &str) -> u32 {
    0
}

fn token_count(a: Matrix2<f64>, b: Vector2<f64>, x: Vector2<f64>) -> Option<u32> {
    let a = Matrix2::from_vec(a.iter().map(|num| *num as u32).collect());
    let b = Vector2::from_vec(b.iter().map(|num| *num as u32).collect());
    let x = Vector2::from_vec(x.iter().map(|num| *num as u32).collect());

    if a * x == b {
        return Some(3 * x[0] + x[1]);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
