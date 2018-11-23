use common::{Dim, List};
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

    pub fn step(&mut self, dt: f32) {
        for b in self.balls.iter_mut() {
            b.step(dt);
        }
        self.time += dt as f64;
    }
}
