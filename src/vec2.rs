use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::str::FromStr;
use std::fmt;
use std::num;

/// 2D vector in cartesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    /// component of vector
    pub x: f64,
    /// component of vector
    pub y: f64,
}

impl Vec2 {
    /// Constructs a new `Vec2`.
    pub fn new<I: Into<f64>>(x: I, y: I) -> Vec2 {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
    /// Constructs a new `Vec2` from polar coordinates $(r, \theta)$.
    pub fn from_polar<I: Into<f64>>(r: I, theta: I) -> Vec2 {
        let (r, theta) = (r.into(), theta.into());
        Vec2::new(r * f64::cos(theta), r * f64::sin(theta))
    }
    /// Create a zero `Vec2`
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    /// Scalar product
    pub fn dot(self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
    /// Orthogonal vector
    pub fn cross(self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }
    /// Area of parallelogramm
    pub fn area(self, rhs: Vec2) -> f64 {
        self.dot(rhs.cross())
    }
    /// Vector length
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    /// Unary vector, co-directed with given
    pub fn ort(self) -> Vec2 {
        self / self.len()
    }
    /// Squares of the vector coordinates
    pub fn sqr(self) -> Vec2 {
        self * self
    }
    /// Square root of vector coordinates
    pub fn sqrt(self) -> Vec2 {
        Vec2::new(self.x.sqrt(), self.y.sqrt())
    }
    /// Constructs dual basis for given.
    ///
    /// Dual basis $(\vec{b}_1, \vec{b}_2)$ for basis $(\vec{a}_1, \vec{a}_2)$ satisfies relation
    /// $$\vec{a}_i \cdot \vec{b}_j = \delta_{ij}$$
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a1 = Vec2::new(2, 0);
    /// let a2 = Vec2::new(3, 4);
    ///
    /// let (b1, b2) = Vec2::dual_basis((a1, a2));
    /// assert_eq!(b1, Vec2::new(0.5, -0.375));
    /// assert_eq!(b2, Vec2::new(0.0, 0.25));
    /// ```
    pub fn dual_basis(basis: (Vec2, Vec2)) -> (Vec2, Vec2) {
        let (a, b) = basis;
        let area = a.area(b);
        (b.cross() / area, -a.cross() / area)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * _rhs.x, self.y * _rhs.y)
    }
}

impl<I: Into<f64>> Mul<I> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: I) -> Vec2 {
        let _rhs = _rhs.into();
        Vec2::new(self.x * _rhs, self.y * _rhs)
    }
}

impl<I: Into<f64>> Div<I> for Vec2 {
    type Output = Self;

    fn div(self, _rhs: I) -> Vec2 {
        let _rhs = _rhs.into();
        if _rhs == 0.0 {
            panic!("Can't divide by zero!");
        }
        Vec2::new(self.x / _rhs, self.y / _rhs)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

impl FromStr for Vec2 {
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

    #[test]
    fn vec2_mul() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(3, 6);
        let r = a * 3;
        assert_eq!(r, b);
    }

    #[test]
    #[should_panic]
    fn vec2_div() {
        let a = Vec2::new(1, 2);
        let _ = a / 0.0;
    }

    #[test]
    fn vec2_from_polar() {
        let a = Vec2::new(3, 4);
        let b = Vec2::from_polar(5.0, f64::atan2(4.0, 3.0));
        assert!((a - b).len() < 1e-10);
    }

    #[test]
    fn vec2_add() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-3, 6);
        let c = Vec2::new(-2, 8);
        assert_eq!(a + b, c);
    }

    #[test]
    fn vec2_sub() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-3, 6);
        let c = Vec2::new(4, -4);
        assert_eq!(a - b, c);
    }

    #[test]
    fn vec2_dot() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-3, 6);
        let c = 9.0;
        assert_eq!(a.dot(b), c);
        assert_eq!(b.dot(a), c);
    }

    #[test]
    fn vec2_area() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-3, 6);
        let c = 12.0;
        assert_eq!(a.area(b), c);
        assert_eq!(b.area(a), -c);
    }

    #[test]
    fn vec2_cross_z() {
        let a = Vec2::new(1, 2);
        let b = 2.0;
        let c = Vec2::new(4, -2);
        assert_eq!(a.cross() * b, c);
    }

    #[test]
    fn vec2_neg() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-1, -2);
        assert_eq!(-a, b);
    }

    #[test]
    fn vec2_parse() {
        let a: Vec2 = "1 2".parse().unwrap();
        assert_eq!(a, Vec2::new(1, 2));
    }
}
