use std::ops::Add;


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn from_u(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize
        }
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn euclid_dist(&self, b: Self) -> Self {
        Self {
            x: (b.x - self.x).abs(),
            y: (b.y - self.y).abs()
        }
    }

    pub fn scale_by(&self, factor: isize) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor
        }
    }

    pub fn components(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a = Point::new(-1, 0);
        let b = Point::from_u(2, 1);
        assert_eq!(a + b, Point::new(1, 1));
    }
}
