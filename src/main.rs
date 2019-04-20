extern crate ecs;

use cgmath::Vector2;
use ecs::system_impl::{clear, flush, init_gl, render_system};
use ecs::types::component::Component;
use ecs::types::component_store::ComponentStore;
use ecs::types::component_type;
use ecs::World;

fn main() {
    let mut world: World = World::new();
    world.add_system(render_system());
    world.add_init_system(init_gl());
    world.add_before(clear());
    world.add_after(flush());

    let index = world.entity.push(Component::RIGID | Component::SHAPE);
    world.push(component_type::Shape::Circle { r: 0.3 }, index);
    world.push(
        component_type::Rigid {
            r: Vector2::new(-0.5, 0.0),
            ..Default::default()
        },
        index,
    );

    let index = world.entity.push(Component::RIGID | Component::SHAPE);
    world.push(component_type::Shape::Rectangle { w: 0.3, h: 0.4 }, index);
    world.push(
        component_type::Rigid {
            r: Vector2::new(0.5, 0.0),
            ..Default::default()
        },
        index,
    );

    world.init();
    world.run();
}
