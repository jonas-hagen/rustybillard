use common::{Dim, List, Vec3};
use na;
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

    /// Reflect a ball at a plane defined by normal vector.
    pub fn reflect(&mut self, n: &na::Unit<Vec3>) {
        let v = self.v;
        let n = n.unwrap();
        self.v = v - 2.0 * (v.dot(&n)) * n;
    }

    pub fn step(&mut self, dt: f32) {
        self.x += dt * self.v;
    }

    pub fn dist(&self, other: &Ball) -> Dim {
        let diff = self.x - other.x;
        na::norm(&diff)
    }
}

#[cfg(test)]
mod test {
    use common::{Vec3};
    use super::*;

    #[test]
    fn test_reflection() {
        let mut ball = Ball::new(Vec3::zeros(), Vec3::x());
        ball.reflect(&Vec3::x_axis());
        assert_eq!(ball.v, -Vec3::x());
        assert_eq!(ball.x, Vec3::zeros());
    }
}
