use std::{collections::HashSet, fs::{read_to_string, File}, io::{BufWriter, Write}, ops::{Index, IndexMut}};
use shared::Point;

pub fn part_01(path: &str) -> isize {
    let source = read_to_string(path).unwrap();
    const SECONDS: isize = 100;

    let (width, height, mut robots) = parse_data(&source);
    let mut quads = [0_isize; 4];

    robots.iter_mut().for_each(|robot| {
        let temp = robot.pos + robot.velocity.scale_by(SECONDS);
        robot.pos = Point::new(
            temp.x % width,
            temp.y % height
        );

        if let Some(idx) = check_quadrant(robot.pos, width, height) {
            quads[idx] += 1;
        }
        
    });

    quads[0] * quads[1] * quads[2] * quads[3]
}

pub fn part_02(path: &str) -> isize {
    let source = read_to_string(path).unwrap();

    let (width, height, mut robots) = parse_data(&source);

    let template = (".".repeat(width as usize) + "\n").repeat(height as usize);
    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(file);

    for i in 1..=10_000 {
        robots.iter_mut().for_each(|robot| {
            let temp = robot.pos + robot.velocity;
            robot.pos = Point::new(
                temp.x % width,
                temp.y % height
            )
        });
        
        let mut copy = template.to_owned().as_bytes().to_owned();

        for robot in &robots {
            // translate position to idx
            let idx = (robot.pos.y * (width + 1) + robot.pos.x) as usize;
            copy[idx] = b'#';
        }
        
        writer.write(format!("{}\n", i).as_bytes()).unwrap();
        writer.write(&copy).unwrap();
        
    }

    writer.flush().unwrap();

    0
}

fn check_quadrant(pos: Point, width: isize, height: isize) -> Option<usize> {
    let w_mid = width / 2;
    let h_mid = height / 2;

    let quads = [
        (w_mid+1..width, 0..h_mid),
        (0..w_mid, 0..h_mid),
        (0..w_mid, h_mid+1..height),
        (w_mid+1..width, h_mid+1..height)
    ];

    quads.iter()
        .position(|quad| {
            quad.0.contains(&pos.x) && quad.1.contains(&pos.y)
        })
}

fn parse_data(source: &str) -> (isize, isize, Vec<Robot>) {
    let mut data_iter = source.lines();

    let (width, height) = data_iter.next().unwrap()
        .split_once(',').unwrap();
    let width = width.parse().unwrap();
    let height = height.parse().unwrap();

    let robots = data_iter.map(|line| {
        let (position, velocity) = line.split_once(' ').unwrap();

        let (pos_x, pos_y) = position[2..].split_once(',').unwrap();
        let pos_x = pos_x.parse().unwrap();
        let pos_y = pos_y.parse().unwrap();

        let (v_x, v_y) = velocity[2..].split_once(',').unwrap();
        let v_x = v_x.parse().unwrap();
        let v_x = if v_x < 0 {
            width + v_x
        } else {
            v_x
        };

        let v_y = v_y.parse().unwrap();
        let v_y = if v_y < 0 {
            height + v_y
        } else {
            v_y
        };

        Robot {
            pos: Point::new(pos_x, pos_y),
            velocity: Point::new(v_x, v_y)
        }
    }).collect();

    (width, height, robots)
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Point,
    velocity: Point
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
