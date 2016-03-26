//! Simple library to work with 2D vectors and points
#![warn(missing_docs)]

pub mod vec2;
pub mod vec3;
pub mod point;
pub mod traits;
// re-export
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use point::*;
pub use traits::*;
