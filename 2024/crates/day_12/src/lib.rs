use std::collections::HashSet;

use shared::{DefaultGrid, Direction, Point};

/// # Errors
/// 
/// Returns an error if the input cannot be parsed into a grid.
pub fn solve(input: &str) -> Result<(usize, usize), String> {
    let grid = DefaultGrid::try_from(input).map_err(|e| format!("Failed to parse input: {e}"))?;
    let mut part_one: usize = 0;
    let mut part_two: usize = 0;
    let mut visited = HashSet::new();

    for (point, plant) in grid.iter_cells() {
        if visited.contains(&point) {
            continue;
        }

        let check = |p: Point| grid.at(p) == Some(plant);

        // flood fill
        let mut area = 0;
        let mut sides = 0;
        let mut perimeter = 0;

        let mut edges = HashSet::new();
        let mut queue = Vec::from([point]);
        visited.insert(point);

        while let Some(point) = queue.pop() {
            area += 1;

            for dir in Direction::CARDINAL_DIRS {
                let neighbor = point + dir;

                if check(neighbor) {
                    if visited.insert(neighbor) {
                        queue.push(neighbor);
                    }
                } else {
                    edges.insert((point, dir));
                    perimeter += 1;
                }
            }
        }

        for &(point, dir) in &edges {
            let r = dir.clockwise();
            let l = dir.counter_clockwise();

            sides += usize::from(!check(point + l) || check(point + l + dir));
            sides += usize::from(!check(point + r) || check(point + r + dir));
        }

        part_one += area * perimeter;
        part_two += area * (sides / 2);
    }

    Ok((part_one, part_two))
}

/// # Errors
///
/// Will return Err if the input cannot be parsed into a grid.
pub fn part_01(input: &str) -> Result<usize, String> {
    let (p1, _) = solve(input)?;
    Ok(p1)
}

/// # Errors
///
/// Will return Err if the input cannot be parsed into a grid.
pub fn part_02(input: &str) -> Result<usize, String> {
    let (_, p2) = solve(input)?;
    Ok(p2)
}
