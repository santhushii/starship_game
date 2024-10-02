use bevy::prelude::*;
use system::{box_movement, move_laser, detect_laser_collision, setup};
use input::LaserTypeTracker;

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LaserTypeTracker::default()) // Track which laser type to shoot
        .insert_resource(component::GameTimer(None, false)) // Game timer resource
        .insert_resource(component::ShipLives(5)) // 5 ship lives
        .add_startup_system(setup) // Setup entities as a system
        .add_system(box_movement) // Box movement system
        .add_system(input::ship_movement) // Ship movement system
        .add_system(input::rotate_ship_on_click) // Rotate ship on mouse click
        .add_system(input::shoot_laser) // Shoot laser on spacebar click
        .add_system(move_laser) // Laser movement system
        .add_system(detect_laser_collision) // Laser and box collision system
        .run();
}
