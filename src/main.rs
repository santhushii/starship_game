use bevy::prelude::*;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::GameTimer(None, false)) // Add game timer resource
        .insert_resource(component::ShipLives(5)) // Add resource for 5 ship lives
        .add_startup_system(system::setup) // Set up the initial entities
        .add_system(system::box_movement) // Add box movement system
        .add_system(input::ship_movement) // Add ship movement system
        .add_system(input::rotate_ship_on_click) // Add mouse click ship rotation system
        .add_system(input::shoot_laser) // Add laser shooting system
        .add_system(system::move_laser) // Add laser movement system
        .add_system(system::detect_laser_collision) // Add laser-box collision system
        .add_system(input::check_end_point_reached) // Check if ship reaches end point
        .add_system(system::update_timer_display) // Display the timer on the screen
        .add_system(input::detect_collision_and_spawn_fireballs) // Add collision detection and fireball system
        .run();
}
