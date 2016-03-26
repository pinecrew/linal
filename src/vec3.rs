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
    pub z: f64,
}

impl Vec3 {
    /// Constructs a new `Vec3`.
    pub fn new<I: Into<f64>>(x: I, y: I, z: I) -> Vec3 {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }
    /// Constructs a new `Vec3` from spherical coordinates $(r, \theta, \phi)$.
    pub fn from_spherical<I: Into<f64>>(r: I, theta: I, phi: I) -> Vec3 {
        let (r, theta, phi) = (r.into(), theta.into(), phi.into());
        Vec3::new(r * f64::sin(theta) * f64::cos(phi),
                  r * f64::sin(theta) * f64::sin(phi),
                  r * f64::cos(theta))
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
    /// Constructs dual basis for given.
    ///
    /// Dual basis $(b_1, b_2)$ for basis $(a_1, a_2)$ satisfies relation
    /// $$a_i \cdot b_j = \delta_{ij}$$
    ///
    /// # Example
    /// ```
    /// use linal::Vec3;
    ///
    /// let a1 = Vec3::new(2.0, 0.0, 0.0);
    /// let a2 = Vec3::new(3.0, 4.0, 0.0);
    /// let a3 = Vec3::new(3.0, 4.0, 5.0);
    ///
    /// let (b1, b2, b3) = Vec3::dual_basis((a1, a2, a3));
    /// assert_eq!(b1, Vec3::new(0.5, -0.375, 0.0));
    /// assert_eq!(b2, Vec3::new(0.0, 0.25, -0.2));
    /// assert_eq!(b3, Vec3::new(0.0, 0.0, 0.2));
    /// ```
    pub fn dual_basis(basis: (Vec3, Vec3, Vec3)) -> (Vec3, Vec3, Vec3) {
        let (a, b, c) = basis;
        let triple_prod = a.cross(b).dot(c);

        (b.cross(c) / triple_prod,
         c.cross(a) / triple_prod,
         a.cross(b) / triple_prod)
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

impl<I: Into<f64>> Mul<I> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: I) -> Vec3 {
        let _rhs = _rhs.into();
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl<I: Into<f64>> Div<I> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: I) -> Vec3 {
        let _rhs = _rhs.into();
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

impl Cross for Vec3 {
    type Output = Self;

    fn cross(self, rhs: Vec3) -> Self {
        Self::new(self.y * rhs.z - self.z * rhs.y,
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
    use traits::Cross;

    #[test]
    fn vec3_mul() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(3, 6, 9);
        let r = a * 3;
        assert_eq!(r, b);
    }

    #[test]
    #[should_panic]
    fn vec3_div() {
        let a = Vec3::new(1, 2, 3);
        let _ = a / 0;
    }

    #[test]
    fn vec3_from_spherical() {
        use std::f64::consts::PI;
        let a = Vec3::from_spherical(5.0, PI / 2.0, 3f64.atan2(4.0));
        let b = Vec3::new(4, 3, 0);
        assert!((a - b).len() < 1e-10);
    }

    #[test]
    fn vec3_add() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(-3, 6, 4);
        let c = Vec3::new(-2, 8, 7);
        assert_eq!(a + b, c);
    }

    #[test]
    fn vec3_sub() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(-3, 6, 4);
        let c = Vec3::new(4, -4, -1);
        assert_eq!(a - b, c);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(-3, 6, 4);
        let c = 21.0;
        assert_eq!(a.dot(b), c);
        assert_eq!(b.dot(a), c);
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3::new(4, 0, 0);
        let b = Vec3::new(3, 5, 0);
        let c = Vec3::new(0, 0, 20);
        assert_eq!(a.cross(b), c);
        assert_eq!(b.cross(a), -c);
    }

    #[test]
    fn vec3_neg() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(-1, -2, -3);
        assert_eq!(-a, b);
    }

    #[test]
    fn vec3_parse() {
        let a: Vec3 = "1 2 3".parse().unwrap();
        assert_eq!(a, Vec3::new(1, 2, 3));
    }
}
