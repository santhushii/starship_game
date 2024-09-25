use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::ExplosionTimer(None)) // Add explosion timer resource
        .add_startup_system(system::setup) // Set up the initial entities
        .add_system(system::box_movement) // Add box movement system
        .add_system(input::ship_movement) // Add ship movement system
        .add_system(input::detect_collision_and_explode) // Add collision detection system
        .add_system(input::fireball_despawn) // Add fireball despawn system
        .run();
}
