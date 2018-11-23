extern crate nalgebra as na;
extern crate rand;

mod ball;
mod world;
mod stats;

pub mod common {
    use na::Vector3;
    pub type Dim = f32;
    pub type Vec3 = Vector3<Dim>;
    pub type List<T> = Vec<T>;
}

pub use ball::Ball;
pub use world::World;
pub use stats::*;
