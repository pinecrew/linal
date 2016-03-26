use std::ops::{Add, Sub, Neg};
use std::cmp::PartialEq;
use std::str::FromStr;
use std::fmt;
use std::num;
use traits::Cross;
use vec2::*;

/// 2D point in cortesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// component of point
    pub x: f64,
    /// component of point
    pub y: f64,
}

impl Point {
    /// Constructs a new `Point`
    ///
    /// #Examples
    ///
    /// ```
    /// use linal::Point;
    /// // ..
    /// // create `Point` with coords (1.5, 3.4)
    /// let a: Point = Point::new(1.5, 3.4);
    /// // return: a = 1.5 3.4
    /// println!("a = {}", a);
    /// ```
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    /// Constructs a new `Point` from polar coordinates $(r, \theta)$.
    pub fn from_polar(r: f64, theta: f64) -> Point {
        Point::new(r * f64::cos(theta), r * f64::sin(theta))
    }
    /// Constructs a zero `Point`
    ///
    /// #Examples
    ///
    /// ```
    /// use linal::Point;
    /// // ...
    /// // create a zero `Point`
    /// let a = Point::zero();
    /// // create b `Point`
    /// let b = Point::new(0.0, 0.0);
    /// // a == b
    /// assert_eq!(a, b)
    /// ```
    pub fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
    /// Construct `Point` from given `Vec2`
    pub fn from_vec2(v: Vec2) -> Point {
        Point::new(v.x, v.y)
    }
    /// Return radius-vector for 'Point'
    pub fn position(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Self;

    fn add(self, _rhs: Vec2) -> Self {
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;

    fn sub(self, _rhs: Vec2) -> Self {
        Point::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Sub for Point {
    type Output = Vec2;

    fn sub(self, _rhs: Point) -> Self::Output {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let x: f64 = try!(words[0].parse());
        let y: f64 = try!(words[1].parse());
        Ok(Self::new(x, y))
    }
}

#[cfg(test)]
mod linal_test {
    use super::*;
    use vec2::*;

    #[test]
    fn point_vec2_add() {
        let a = Point::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Point::new(-2.0, 8.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn point_vec2_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Point::new(4.0, -4.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn point_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-3.0, 6.0);
        let c = Vec2::new(4.0, -4.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn point_neg() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-1.0, -2.0);
        assert_eq!(-a, b);
    }

    #[test]
    fn point_parse() {
        let a: Point = "1 2".parse().unwrap();
        assert_eq!(a, Point::new(1.0, 2.0));
    }
}
