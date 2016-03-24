extern crate linal;
use linal::{Vec2, Point};

fn main() {
    let vec = Vec2::new(3.3, 5.5);
    // initialize Point
    let a = Point::new(2.3, 4.5);
    println!("a = ({})", a);
    // from Vec2 to Point
    let b = Point::from_vec2(vec);
    println!("convert Vec2({}) to Point({})", vec, b);
    // initialize zero Point
    println!("Point::zero() = ({})", Point::zero());
    // convert Point to Vec2
    println!("({}).position = ({})", a, a.position());
    // Point + Vec2 = Point
    println!("({}) + ({}) = ({})", a, vec, a + vec);
    // Point - Vec2 = Point
    println!("({}) - ({}) = ({})", b, vec, a - vec);
    // Point - Point = Vec2
    println!("({}) - ({}) = ({})", b, b, a - b);
    // negative operation
    println!("({}).neg() = ({})", a, -a);
    // equality operation
    println!("({}) == ({}) = {}", a, b, a == b);
    let point = "3.5 2.8";
    // parse point from string
    println!("{} --> ({})", point, point.parse::<Point>().unwrap());
}