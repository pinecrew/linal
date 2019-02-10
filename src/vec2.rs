//! Vectors on a plane.
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{Index, IndexMut};
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
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// // create `Vec2` with int
    /// let a = Vec2::new(10, 20);
    /// // create `Vec2` with float
    /// let b = Vec2::new(3.5, 2.5);
    /// // Supported types implemented for trait Into (with convertion to f64)
    /// ```
    pub fn new<I: Into<f64>>(x: I, y: I) -> Vec2 {
        Vec2 {
            x: x.into(),
            y: y.into(),
        }
    }
    /// Constructs a new `Vec2` from polar coordinates $(r, \theta)$.
    ///
    /// # Example
    /// ```
    /// use std::f64::consts::PI;
    /// use linal::Vec2;
    ///
    /// // calculation error
    /// let eps = 1E-15;
    /// // Create `Vec2` use polar coordinates
    /// let v = Vec2::from_polar(2.0, PI / 2.0);
    /// assert!(v.x < eps && v.y - 2.0 < eps);
    /// ```
    pub fn from_polar<I: Into<f64>>(r: I, theta: I) -> Vec2 {
        let (r, theta) = (r.into(), theta.into());
        Vec2::new(r * f64::cos(theta), r * f64::sin(theta))
    }
    /// Create a zero `Vec2`
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// // create zero `Vec2`
    /// let zero = Vec2::zero();
    /// assert_eq!(zero, Vec2::new(0, 0));
    /// ```
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    /// Scalar product
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a = Vec2::new(1, 2);
    /// let b = Vec2::new(3, 4);
    /// // The scalar production of `a` by `b`
    /// let r = a.dot(b);
    /// assert_eq!(r, 11.0);
    /// ```
    pub fn dot(self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
    /// Orthogonal vector
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a = Vec2::new(2, 2);
    /// let b = Vec2::new(2, -2);
    /// // create orthogonal vector with same length
    /// // rotated in clockwise direction
    /// //             y ^
    /// //               |
    /// //               |
    /// //             2 - ...... /a
    /// //               |     //  :
    /// //             1 -   //    :
    /// //    -2   -1    | //      :
    /// //  -- | -- | -- 0 -- | -- | ---->
    /// //               | \\   1  : 2     x
    /// //               - -1\\    :
    /// //               |     \\  :
    /// //               - -2.....\b
    /// let c = a.cross();
    /// assert_eq!(b, c);
    /// ```
    pub fn cross(self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }
    /// Area of parallelogramm
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a = Vec2::new(2, 0);
    /// let b = Vec2::new(1, 2);
    /// // Calculate the area of the parallelogram formed by the vectors
    /// // y ^
    /// //   |
    /// //   |
    /// // 2 -    b .........
    /// //   |   /#########/
    /// // 1 -  /#  area #/
    /// //   | /#########/ 
    /// //   0 -- | -- a ---->
    /// //        1    2     x
    /// let area = a.area(b);
    /// assert_eq!(area, 4.0);
    /// ```
    pub fn area(self, rhs: Vec2) -> f64 {
        self.dot(rhs.cross())
    }
    /// Vector length
    /// 
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let vec = Vec2::new(2, 0);
    /// // Calculate vector length
    /// let len1 = vec.len();
    /// let len2 = (-vec.cross()).len();
    /// assert!(len1 == len2 && len1 == 2.0);
    /// ```
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    /// Unary vector, co-directed with given
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a = Vec2::new(2, 0);
    /// // Calculate unary vector from `a`
    /// let b = a.ort();
    /// assert_eq!(b, Vec2::new(1, 0));
    /// ```
    pub fn ort(self) -> Vec2 {
        self / self.len()
    }
    /// Squares of the vector coordinates
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    ///
    /// let a = Vec2::new(2, 3);
    /// let b = Vec2::new(4, 9);
    /// // Calculate square of `a`
    /// let c = a.sqr();
    /// assert_eq!(b, c);
    /// ```
    pub fn sqr(self) -> Vec2 {
        self * self
    }
    /// Square root of vector coordinates
    ///
    /// # Example
    /// ```
    /// use linal::Vec2;
    /// 
    /// let a = Vec2::new(2, 3);
    /// let b = Vec2::new(4, 9);
    /// // Calculate squre root of `b`
    /// let c = b.sqrt();
    /// assert_eq!(a, c);
    /// ```
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

    // need for op_default & op_assign
    fn size(&self) -> usize { 2 }
}

op_default!(add, Add, +=, Vec2);
op_default!(sub, Sub, -=, Vec2);
op_default!(mul, Mul, *=, Vec2);
op_default!(f64, mul, Mul, *=, Vec2);
op_default!(f64, div, Div, /=, Vec2);
op_assign!(add_assign, AddAssign, +=, Vec2);
op_assign!(sub_assign, SubAssign, -=, Vec2);
op_assign!(mul_assign, MulAssign, *=, Vec2);
op_assign!(f64, mul_assign, MulAssign, *=, Vec2);
op_assign!(f64, div_assign, DivAssign, /=, Vec2);

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            i => panic!("Index {} out of [0, 1] range", i)
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            i => panic!("Index {} out of [0, 1] range", i)
        }
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
        let x: f64 = words[0].parse()?;
        let y: f64 = words[1].parse()?;
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
        let mut z = a;
        let mut x = a;
        z *= 3;
        x *= b;
        assert_eq!(r, b);
        assert_eq!(z, b);
        assert_eq!(x, Vec2::new(3, 12));
    }

    #[test]
    fn vec2_div() {
        let a = Vec2::new(10, 20);
        let b = Vec2::new(1, 2);
        let mut z = a;
        z /= 10;
        assert_eq!(a / 10, b);
        assert_eq!(z, b);
    }

    #[test]
    fn vec2_div_inf() {
        let a = Vec2::new(1, 2);
        let b = a / 0.0;
        assert!(b.x.is_infinite() && b.y.is_infinite());
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

        let mut z = a;
        z += b;
        assert_eq!(z, c);
    }

    #[test]
    fn vec2_sub() {
        let a = Vec2::new(1, 2);
        let b = Vec2::new(-3, 6);
        let c = Vec2::new(4, -4);
        let mut z = a;
        z -= b;
        assert_eq!(a - b, c);
        assert_eq!(z, c);
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
    fn vec2_index() {
        let a = Vec2::new(1, 2);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
    }

    #[test]
    #[should_panic]
    fn vec2_index_out_of_range() {
        let a = Vec2::new(1, 2);
        let _ = a[10];
    }

    #[test]
    fn vec2_index_mut() {
        let mut a = Vec2::zero();
        for i in 0..2 {
            a[i] = (i as f64 + 1.0).powi(2);
        }
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 4.0);
    }

    #[test]
    #[should_panic]
    fn vec2_index_mut_out_of_range() {
        let mut a = Vec2::zero();
        a[10] = 10.0;
    }

    #[test]
    fn vec2_parse() {
        let a: Vec2 = "1 2".parse().unwrap();
        assert_eq!(a, Vec2::new(1, 2));
    }
}
