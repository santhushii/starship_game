use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::ExplosionTimer(None)) // Add explosion timer resource
        .add_startup_system(system::setup) // Initialize setup system
        .add_system(input::ship_movement) // Ship movement based on keyboard input
        .add_system(input::rotate_ship_towards_mouse) // Ship rotation based on mouse input
        .add_system(input::detect_collision_and_explode) // Handle collisions and explosions
        .add_system(input::fireball_despawn) // Fireball despawn logic after explosion
        .add_system(system::box_movement) // Box movement system
        .run();
}
