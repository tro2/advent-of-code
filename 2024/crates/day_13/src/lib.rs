use std::fs::read_to_string;

use nalgebra::{Matrix2, Vector2};

type Coord = (f64, f64);
type Coord32 = (u32, u32);

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();
    let mut tokens = 0;

    for group in source.split("\n\n") {
        let data: Vec<(u32, u32)> = group
            .lines()
            .map(|line| {
                let (_, numstr) = line.split_once(':').unwrap();
                let (a, b) = numstr.split_once(',').unwrap();
                (a[3..].parse().unwrap(), b[3..].parse().unwrap())
            })
            .collect();

        if data[0].0 == data[1].0 {
            // infinitely many solutions, x's are same
            tokens += solve_x_eq(data[0], data[1], data[2]);
        } else if data[0].1 == data[1].1 {
            // infinitely many solutions, y's are same
            tokens += solve_y_eq(data[0], data[1], data[2]);
        } else {
            let data: Vec<(f64, f64)> = data.iter().map(|(x, y)| (*x as f64, *y as f64)).collect();
            tokens += solve_lin_eq(data[0], data[1], data[2]);
        }
    }
    tokens
}

pub fn part_02(path: &str) -> u32 {
    0
}

fn solve_x_eq(a: Coord32, b: Coord32, prize: Coord32) -> u32 {
    if prize.0 % a.0 != 0 {
        return 0;
    }

    let mut cost = 1000;

    for c_1 in 1..=100 {
        for c_2 in 1..=100 - c_1 {
            if c_1 * a.1 + c_2 * b.1 == prize.1 {
                let c_t = 3 * c_1 + c_2;
                if c_t < cost {
                    cost = c_t;
                }
            }
        }
    }

    if cost == 1000 {
        0
    } else {
        cost
    }
}

fn solve_y_eq(a: Coord32, b: Coord32, prize: Coord32) -> u32 {
    if prize.1 % a.1 != 0 {
        return 0;
    }

    let mut cost = 1000;

    for c_1 in 1..=100 {
        for c_2 in 1..=100 - c_1 {
            if c_1 * a.0 + c_2 * b.0 == prize.0 {
                let c_t = 3 * c_1 + c_2;
                if c_t < cost {
                    cost = c_t;
                }
            }
        }
    }

    if cost == 1000 {
        0
    } else {
        cost
    }
}

fn solve_lin_eq(a: Coord, b: Coord, prize: Coord) -> u32 {
    let coefficients = Matrix2::new(a.0, b.0, a.1, b.1);

    let end = Vector2::new(prize.0, prize.1);
    let decomp = coefficients.lu();
    if let Some(x) = decomp.solve(&end) {
        if let Some(count) = token_count(coefficients, end, x) {
            return count;
        }
    }
    return 0;
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
