use bevy::prelude::*;
use system::{
    animate_fireball, box_movement, box_ship_collision, check_end_point_reached, detect_laser_collision, detect_starship_box_collision, move_laser, setup, update_timer_display
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
        .add_systems(Startup, setup) // Use new system registration syntax for startup systems
        .add_systems(Update, (
            box_movement, // Register box movement system separately
            box_ship_collision, // Register collision detection system separately
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
