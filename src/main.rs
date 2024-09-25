use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::GameTimer(None, false)) // Add game timer resource
        .add_startup_system(system::setup) // Set up the initial entities
        .add_system(system::box_movement) // Add box movement system
        .add_system(input::ship_movement) // Add ship movement system
        .add_system(input::rotate_ship_on_click) // Add mouse click ship rotation system
        .add_system(input::check_end_point_reached) // Check if ship reaches end point
        .add_system(input::track_game_timer) // Track and print game timer
        .add_system(input::detect_collision_and_reset) // Add collision detection and reset system
        .run();
}
