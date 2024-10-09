use bevy::prelude::*;
use system::{
    box_movement, move_laser, detect_laser_collision, setup, animate_fireball,
    detect_starship_box_collision, check_end_point_reached, update_timer_display,
};
use input::{LaserTypeTracker, ship_movement, rotate_ship_on_click, shoot_laser};

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LaserTypeTracker::default()) // Track which laser type to shoot
        .insert_resource(component::GameTimer(None, false)) // Game timer resource
        .insert_resource(component::ShipLives(5)) // 5 ship lives
        .add_startup_system(setup) // Use new system registration syntax for startup systems
        .add_systems(Update, (
            box_movement, // Register systems using new syntax
            ship_movement,
            rotate_ship_on_click,
            shoot_laser,
            move_laser,
            detect_laser_collision,
            update_timer_display,
            detect_starship_box_collision,
            check_end_point_reached,
            animate_fireball,
        ))
        .run();
}
