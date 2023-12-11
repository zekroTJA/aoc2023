use crate::vector::Vector;
use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Default, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn is_negative(&self) -> bool {
        self.x.is_negative() || self.y.is_negative()
    }

    pub fn is_null(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn manhattan_distance(&self, to: Pos) -> usize {
        (to.y - self.y).unsigned_abs() + (to.x - self.x).unsigned_abs()
    }
}

impl Vector for Pos {
    type Output = Pos;

    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt().abs()
    }

    fn flatten(&self) -> Self::Output {
        let (mut x, mut y) = (self.x, self.y);
        if x != 0 {
            x /= x.abs();
        }
        if y != 0 {
            y /= y.abs();
        }
        (x, y).into()
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> From<(T, T)> for Pos
where
    T: Into<isize>,
{
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
