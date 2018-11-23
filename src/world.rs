use common::{Dim, List, Vec3};
use Ball;

#[derive(Debug)]
pub struct World {
    size: Dim,
    balls: List<Ball>,
    time: f64,
}

impl World {
    pub fn new_random(size: Dim, n_balls: i64) -> World {
        let mut balls = List::new();

        for _ in 0..n_balls {
            balls.push(Ball::new_random(size));
        }

        World {
            size,
            balls,
            time: 0.0,
        }
    }

    pub fn reflect_at_walls(&mut self) {
        for b in self.balls.iter_mut() {
            let x = b.x;
            let lower = Ball::RADIUS;
            let upper = self.size - Ball::RADIUS;
            if x.x < lower || x.x > upper {
                b.reflect(&Vec3::x_axis());
            }
            if x.y < lower || x.y > upper {
                b.reflect(&Vec3::y_axis());
            }
            if x.z < lower || x.z > upper {
                b.reflect(&Vec3::z_axis());
            }
        }
    }

    pub fn resolve_collisions(&mut self) -> i32 {
        let mut num_collisions = 0;
        for i in 1..self.balls.len() {
            let (mut left, mut right) = self.balls.split_at_mut(i);
            // Following unwrap is always ok, because of bounds on i
            let mut b1 = left.last_mut().unwrap();
            for mut b2 in right.iter_mut() {
                if b1.maybe_collide(&mut b2) {
                    num_collisions += 1;
                }
            }
        }
        return num_collisions;
    }

    fn step_balls(&mut self, dt: f32) {
        for b in self.balls.iter_mut() {
            b.step(dt);
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.step_balls(dt);
        self.reflect_at_walls();
        self.resolve_collisions();
        self.time += dt as f64;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::Vec3;

    /// Create world with one ball at the center
    fn world_with_one_ball() -> World {
        let balls = vec![Ball::new(Vec3::new(1.5, 1.5, 1.5), Vec3::x())];
        World {
            size: 3.0,
            balls: balls,
            time: 0.0,
        }
    }

    #[test]
    fn no_reflection() {
        let mut world = world_with_one_ball();
        let x0 = world.balls[0].x.clone();
        let v0 = world.balls[0].v.clone();

        // That sould change nothing
        world.reflect_at_walls();
        assert_eq!(world.balls[0].x, x0);
        assert_eq!(world.balls[0].v, v0);
    }

    #[test]
    fn reflection() {
        let mut world = world_with_one_ball();
        let v0 = world.balls[0].v.clone();

        world.step_balls(0.6);
        let x1 = world.balls[0].x.clone();

        // That sould change the speed only
        world.reflect_at_walls();
        assert_eq!(world.balls[0].x, x1);
        assert_eq!(world.balls[0].v, -v0);
    }

    #[test]
    fn resolve_collisions() {
        let balls = vec![
            Ball::new(Vec3::new(2.0, 5.0, 5.0), Vec3::x()),
            Ball::new(Vec3::new(3.9, 5.0, 5.0), -Vec3::x()),
        ];
        let mut world = World {
            size: 10.0,
            balls: balls,
            time: 0.0,
        };

        let num_colisions = world.resolve_collisions();
        assert_eq!(num_colisions, 1);

        // Collisions should be resolved now
        let num_colisions = world.resolve_collisions();
        assert_eq!(num_colisions, 0);
    }
}
