extern crate linal;
use linal::{Vec3, Cross};

fn main() {
    // initialize three dimension vector
    let a = Vec3::new(2.00, 4.0, 8.0);
    let b = Vec3::new(3.15, 3.0, 3.3);
    // addition
    println!("({}) + ({}) = ({})", a, b, a + b);
    // substraction
    println!("({}) - ({}) = ({})", b, a, b - a);
    let (k, n) = (3.4, 8.0);
    // multiplication by a constant
    println!("({}) * {} = ({})", a, k, a * k);
    // division by a constant
    println!("({}) / {} = ({})", b, k, b / n);
    let (r, theta, phi) = (2.0, 1.57, 3.14);
    // initialize zero vector
    println!("Vec3::zero() = ({})", Vec3::zero());
    // transformation from the polar coordinate system
    println!("from_spherical({}, {}, {}) = ({})",
             r,
             theta,
             phi,
             Vec3::from_spherical(r, theta, phi));
    // construct dual basis
    let a1 = Vec3::new(2.0, 0.0, 0.0);
    let a2 = Vec3::new(3.0, 4.0, 0.0);
    let a3 = Vec3::new(3.0, 4.0, 5.0);
    let (b1, b2, b3) = Vec3::dual_basis((a1, a2, a3));
    println!("dual_basis(({}), ({}), ({})) = (({}), ({}), ({}))",
             a1,
             a2,
             a3,
             b1,
             b2,
             b3);
    // scalar production
    println!("<({}), ({})> = {}", a, b, a.dot(b));
    // ...
    println!("({}).cross({}) = {}", a, b, a.cross(b));
    // vector length
    println!("({}).len() = {}", a, a.len());
    // unary vector, co-directed with given
    println!("({}).ort() = ({})", b, b.ort());
    // squares of the vector coordinates
    println!("({}).sqr() = ({})", a, a.sqr());
    // square root of vector coordinates
    println!("({}).sqrt() = ({})", b, b.sqrt());
    // negative operation
    println!("({}).neg() = ({})", a, -a);
    // equality operation
    println!("({}) == ({}) = {}", a, b, a == b);
    let vector = "3.5 2.8 2.71";
    // parse vector from string
    println!("{} --> ({})", vector, vector.parse::<Vec3>().unwrap());
}
