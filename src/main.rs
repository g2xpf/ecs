extern crate ecs;

use ecs::system_impl::{init_gl, render_system};
use ecs::types::component::Component;
use ecs::types::component_store::ComponentStore;
use ecs::types::component_type;
use ecs::World;

fn main() {
    let mut world: World<f32> = World::new();
    world.add_system(render_system());
    world.add_init_system(init_gl());

    let index = world.entity.push(Component::RIGID | Component::SHAPE);
    world.push(component_type::Shape::Circle { r: 1.4 }, index);
    world.push(component_type::Rigid::default(), index);

    world.init();
    world.run();
}
