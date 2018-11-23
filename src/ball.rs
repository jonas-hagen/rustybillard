use common::{Dim, List, Vec3};
use na::norm;
use rand::Rng;

#[derive(Debug)]
pub struct Ball {
    x: Vec3,
    v: Vec3,
}

impl Ball {
    pub fn new(x: Vec3, v: Vec3) -> Ball {
        return Ball { x, v };
    }

    pub fn new_random(world_size: Dim) -> Ball {
        let mut rng = rand::thread_rng();
        // Create 6 random numbers
        let r: List<Dim> = (0..6).map(|_| rng.gen_range(0.0, world_size)).collect();

        let x = Vec3::new(r[0], r[1], r[2]);
        let v = 0.1 * Vec3::new(r[3], r[4], r[5]);
        Ball::new(x, v)
    }

    pub fn step(&mut self, dt: f32) {
        self.x += dt * self.v;
    }

    pub fn dist(&self, other: &Ball) -> Dim {
        let diff = self.x - other.x;
        norm(&diff)
    }
}
