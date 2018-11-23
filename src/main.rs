extern crate billard;

use billard::World;

fn main() {
    let mut world = World::new_random(100.0, 1);
    println!("Our world: {:#?}", world);
    world.step(1.0);
    println!("Our world: {:#?}", world);
}
