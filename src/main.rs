extern crate billard;

use billard::World;

fn main() {
    let mut world = World::new_random(100.0, 1000);
    let mut n_steps = 0;
    loop {
        world.step(0.1);
        if n_steps % 10 == 0 {
            println!("\n\n\n");
            println!("Time: {}", world.time());
            println!("");
            if let Some(hist) = world.hist_speed() {
                println!("{:}", hist);
                hist.plot_ascii();
            }
        }
        n_steps += 1;
    }
}
