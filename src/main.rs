use bevy::prelude::*;
use system::{box_movement, update_timer_display, move_laser, detect_laser_collision}; // Removed animate_fireball

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(component::GameTimer(None, false)) // Game timer resource
        .insert_resource(component::ShipLives(5)) // 5 ship lives
        .add_startup_system(system::setup) // Setup entities
        .add_system(system::box_movement) // Box movement system
        .add_system(input::ship_movement) // Ship movement system
        .add_system(input::rotate_ship_on_click) // Rotate ship on mouse click
        .add_system(input::shoot_laser) // Shoot laser on spacebar click
        .add_system(system::move_laser) // Laser movement system
        .add_system(system::detect_laser_collision) // Laser and box collision system
        .add_system(input::check_end_point_reached) // Check if ship reaches the end point
        .add_system(system::update_timer_display) // Display game timer system
        .add_system(system::animate_box_destruction) // Animate box destruction system
        .run();
}
