use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::str::FromStr;
use std::fmt;
use std::num;
use traits::Cross;

/// 3D vector in cartesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    /// component of vector
    pub x: f64,
    /// component of vector
    pub y: f64,
    /// component of vector
    pub z: f64
}

impl Vec3 {
    /// Constructs a new `Vec3`.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    /// Constructs a new `Vec3` from spherical coordinates $(r, \theta, \phi)$.
    pub fn from_spherical(r: f64, theta: f64, phi: f64) -> Vec3 {
        Vec3::new(
            r * f64::sin(theta) * f64::cos(phi), 
            r * f64::sin(theta) * f64::sin(phi),
            r * f64::cos(theta)
        )
    }
    /// Create a zero `Vec3`
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    /// Scalar product
    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    /// Vector length
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    /// Unary vector, co-directed with given
    pub fn ort(self) -> Vec3 {
        self / self.len()
    }
    /// Squares of the vector coordinates
    pub fn sqr(self) -> Vec3 {
        self * self
    }
    /// Square root of vector coordinates
    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Vec3 {
        if _rhs == 0.0 {
            panic!("Can't divide by zero!");
        }
        Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl FromStr for Vec3 {
    type Err = num::ParseFloatError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let x: f64 = try!(words[0].parse());
        let y: f64 = try!(words[1].parse());
        let z: f64 = try!(words[2].parse());
        Ok(Self::new(x, y, z))
    }
}

#[cfg(test)] 
mod linal_test {
    use super::*;
    // use traits::Cross;

    #[test]
    fn vec3_mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 6.0, 9.0);
        let r = a * 3.0;
        assert_eq!(r, b);
    }

    #[test]
    #[should_panic]
    fn vec3_div() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let _ = a / 0.0;
    }

    #[test]
    #[ignore]
    fn vec3_from_spherical() {
        // need test
    }

    #[test]
    fn vec3_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-3.0, 6.0, 4.0);
        let c = Vec3::new(-2.0, 8.0, 7.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn vec3_sub() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-3.0, 6.0, 4.0);
        let c = Vec3::new(4.0, -4.0, -1.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-3.0, 6.0, 4.0);
        let c = 21.0;
        assert_eq!(a.dot(b), c);
        assert_eq!(b.dot(a), c);
    }

    #[test]
    #[ignore]
    fn vec3_cross() {
        // write test
    }

    #[test]
    #[ignore]
    fn vec3_cross_z() {
        // write test
    }

    #[test]
    fn vec3_neg() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-a, b);
    }

    #[test]
    fn vec3_parse() {
        let a: Vec3 = "1 2 3".parse().unwrap();
        assert_eq!(a, Vec3::new(1.0, 2.0, 3.0));
    }
}