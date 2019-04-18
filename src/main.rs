extern crate ecs;

use ecs::system_impl::{init_gl, render_system};
use ecs::World;

fn main() {
    let mut world = World::new();
    world.add_system(render_system());
    world.add_init_system(init_gl());
    world.init();
    world.run();
}
