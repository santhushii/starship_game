use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::ExplosionTimer(None)) // Add explosion timer resource
        .add_startup_system(system::setup) // No need for .system()
        .add_system(input::ship_movement) // No need for .system()
        .add_system(input::rotate_ship_towards_mouse) // Add the mouse rotation system
        .add_system(input::detect_collision_and_explode) // No need for .system()
        .add_system(input::fireball_despawn) // No need for .system()
        .run();
}
