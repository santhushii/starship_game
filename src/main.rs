use bevy::prelude::*;
use system::update_timer_display;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::GameTimer(None, false)) // Game timer resource
        .insert_resource(component::ShipLives(5)) // 5 ship lives
        .add_startup_system(system::setup) // Setup entities
        .add_system(system::box_movement) // Box movement
        .add_system(input::ship_movement) // Ship movement
        .add_system(input::rotate_ship_on_click) // Ship rotation on mouse click
        .add_system(input::shoot_laser) // Laser shooting
        .add_system(system::move_laser) // Laser movement
        .add_system(system::detect_laser_collision) // Laser and box collision detection
        .add_system(input::check_end_point_reached) // Check if ship reaches end point
        .add_system(system::update_timer_display) // Display timer
        .add_system(system::animate_fireball) // Animate fireball
        .add_system(input::detect_collision_and_spawn_fireballs) // Detect collision between ship and boxes
        .run();
}
