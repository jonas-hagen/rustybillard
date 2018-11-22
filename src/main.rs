extern crate nalgebra as na;
extern crate rand;

use na::{Vector3, norm};
use rand::Rng;

type Dim = f32;
type Vec3 = Vector3<Dim>;
type List<T> = Vec<T>;

#[derive(Debug)]
struct Ball {
    x: Vec3,
    v: Vec3,
}

impl Ball {
    fn new(x: Vec3, v: Vec3) -> Ball {
        return Ball{x,v};
    }

    fn step(&mut self, dt: f32) {
        self.x += dt * self.v;
    }

    fn dist(&self, other: &Ball) -> Dim {
        let diff = self.x - other.x;
        norm(&diff)
    }
}

#[derive(Debug)]
struct World {
    size: Dim,
    balls: List<Ball>,
    time: f64,
}

impl World {
    fn new_random(size: Dim, n_balls: i64) -> World {
        let mut rng = rand::thread_rng();
        let mut balls = List::new();

        for _ in 0..n_balls {
            // Create 6 random numbers
            let r: List<Dim> = (0..6).map(|_| {
                rng.gen_range(0.0, size)
            }).collect();

            let x = Vec3::new(r[0], r[1], r[2]);
            let v = 0.1 * Vec3::new(r[3], r[4], r[5]);
            balls.push(Ball::new(x, v));
        }

        World{size, balls, time: 0.0}
    }

    fn step(&mut self, dt: f32) {
        for b in self.balls.iter_mut() {
            b.step(dt);
        }
        self.time += dt as f64;
    }

}

fn main() {
    let mut world = World::new_random(100.0, 1);
    println!("Our world: {:#?}", world);
    world.step(1.0);
    println!("Our world: {:#?}", world);
}
