extern crate linal;
use linal::{Vec2, Cross};

fn main() {
    // initialize two dimension vector
    let a = Vec2::new(2.00, 4.00);
    let b = Vec2::new(3.15, 3.00);
    // addition
    println!("({}) + ({}) = ({})", a, b, a + b);
    // substraction
    println!("({}) - ({}) = ({})", b, a, b - a);
    let (k, n) = (3.4, 8.0);
    // multiplication by a constant
    println!("({}) * {} = ({})", a, k, a * k);
    // division by a constant
    println!("({}) / {} = ({})", b, n, b / n);
    let (r, theta) = (2.0, 3.14);
    // initialize zero vector
    println!("Vec2::zero() = ({})", Vec2::zero());
    // transformation from the polar coordinate system
    println!("from_polar({}, {}) = ({})",
             r,
             theta,
             Vec2::from_polar(r, theta));
    // construct dual basis
    let a1 = Vec2::new(2.0, 0.0);
    let a2 = Vec2::new(3.0, 4.0);
    let (b1, b2) = Vec2::dual_basis((a1, a2));
    println!("dual_basis(({}), ({})) = (({}), ({}))", a1, a2, b1, b2);
    // scalar product
    println!("<({}), ({})> = {}", a, b, a.dot(b));
    // cross product
    println!("({}).cross({}) = {}", a, b, a.cross(b));
    // cross product with orthogonal vector
    println!("({}).cross({}) = ({})", a, k, a.cross(k));
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
    let vector = "3.5 2.8";
    // parse vector from string
    println!("{} --> ({})", vector, vector.parse::<Vec2>().unwrap());
}
