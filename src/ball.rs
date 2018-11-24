use common::{Dim, List, Vec3};
use na;
use rand::Rng;

#[derive(Debug)]
pub struct Ball {
    pub x: Vec3,
    pub v: Vec3,
}

impl Ball {
    pub const RADIUS: f32 = 1.0;

    pub fn new(x: Vec3, v: Vec3) -> Ball {
        return Ball { x, v };
    }

    pub fn new_random(world_size: Dim) -> Ball {
        let mut rng = rand::thread_rng();
        // Create 6 random numbers
        let r: List<Dim> = (0..6).map(|_| rng.gen_range(0.0, world_size)).collect();

        let x = Vec3::new(r[0], r[1], r[2]);
        let v = Vec3::new(r[3], r[4], r[5]);
        let v = 0.2 * (world_size as f32) * v.normalize();
        Ball::new(x, v)
    }

    /// Reflect a ball at a plane defined by normal vector.
    pub fn reflect(&mut self, n: &na::Unit<Vec3>) {
        let v = self.v;
        let n = n.unwrap();
        self.v = v - 2.0 * (v.dot(&n)) * n;
    }

    /// Collide two balls, changing the momentum of both.
    /// If balls are too far away, then no collision happens.
    /// If projected collision speed is parallel
    /// and follower is slower, then no collision happens.
    /// This is typically a concequence of too large
    /// timesteps in the simulation and balls can 'stick'
    /// to each another.
    /// Retrun value inidcates if collision happend.
    pub fn maybe_collide(&mut self, other: &mut Ball) -> bool {
        let dist = (self.x - other.x).norm();
        if dist > 2.0 * Self::RADIUS {
            // Too far away
            return false;
        }

        let v1 = self.v;
        let v2 = other.v;
        let x = (self.x - other.x) / dist;
        let s1x = x.dot(&v1);
        let s2x = x.dot(&v2);

        if s1x > 0.0 && s2x < 0.0 {
            // Moving away from each another
            return false;
        } else if s1x.signum() == s2x.signum() && s1x > s2x {
            // Projected collision speed is parallel
            // and follower is slower
            return false;
        }

        let v1x = s1x * x;
        let v2x = s2x * x;
        let v1y = v1 - v1x;
        let v2y = v2 - v2x;

        self.v = v2x + v1y;
        other.v = v1x + v2y;
        return true;
    }

    pub fn step(&mut self, dt: f32) {
        self.x += dt * self.v;
    }

    pub fn dist(&self, other: &Ball) -> Dim {
        let diff = self.x - other.x;
        na::norm(&diff)
    }

    pub fn speed(&self) -> f32 {
        return self.v.magnitude();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::Vec3;

    #[test]
    fn test_reflection() {
        let mut ball = Ball::new(Vec3::zeros(), Vec3::x());
        ball.reflect(&Vec3::x_axis());
        assert_eq!(ball.v, -Vec3::x());
        assert_eq!(ball.x, Vec3::zeros());
    }

    #[test]
    fn collision_parallel() {
        let mut b1 = Ball::new(Vec3::zeros(), Vec3::x());
        let mut b2 = Ball::new(Vec3::new(1.9 * Ball::RADIUS, 0.0, 0.0), -Vec3::x());
        println!("First collision...");
        let collision = b1.maybe_collide(&mut b2);
        assert!(collision, "Expected collision but did not happen");
        assert_eq!(b1.v, -Vec3::x());
        assert_eq!(b2.v, Vec3::x());
        assert_eq!(b1.x, Vec3::zeros());

        println!("Second collision...");
        let collision = b1.maybe_collide(&mut b2);
        assert!(!collision, "No collision expected, but did happen");
    }

    #[test]
    fn collision_anti_parallel() {
        let mut b1 = Ball::new(Vec3::zeros(), 2.0 * Vec3::x());
        let mut b2 = Ball::new(Vec3::new(1.9 * Ball::RADIUS, 0.0, 0.0), Vec3::x());
        println!("First collision...");
        let collision = b1.maybe_collide(&mut b2);
        assert!(collision, "Expected collision but did not happen");
        assert_eq!(b1.v, Vec3::x());
        assert_eq!(b2.v, 2.0 * Vec3::x());
        assert_eq!(b1.x, Vec3::zeros());

        println!("Second collision...");
        let collision = b1.maybe_collide(&mut b2);
        assert!(!collision, "No collision expected, but did happen");
    }
    #[test]
    fn collision_far() {
        let mut b1 = Ball::new(Vec3::zeros(), Vec3::x());
        let mut b2 = Ball::new(Vec3::new(2.5 * Ball::RADIUS, 0.0, 0.0), -Vec3::x());
        println!("First collision...");
        let collision = b1.maybe_collide(&mut b2);
        assert!(!collision, "No collision expected, but did happen");
    }
}
