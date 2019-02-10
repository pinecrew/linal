//! Small and simple library to work with 2D and 3D vectors
#![warn(missing_docs)]

#[macro_use]
mod macros;
pub mod vec2;
pub mod vec3;
// re-export
pub use vec2::Vec2;
pub use vec3::Vec3;
