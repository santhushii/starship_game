// main.rs
use bevy::prelude::*;
use system::{
    animate_fireball, box_movement, box_ship_collision, check_end_point_reached,
    detect_laser_collision, detect_starship_box_collision, move_laser, setup, update_timer_display,
};
use input::{LaserTypeTracker, ship_movement, rotate_ship_follow_cursor, shoot_laser};

mod component;
mod system;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds Bevy's default plugins

        // Insert resources
        .insert_resource(LaserTypeTracker::default()) // Track which laser type to shoot
        .insert_resource(component::GameTimer(None, false)) // Game timer resource
        .insert_resource(component::ShipLives(5)) // Initialize with 5 ship lives

        // Use the recommended way to add startup systems
        .add_systems(Startup, setup) // Setup the initial game state

        // Add update systems using the new syntax
        .add_systems(
            Update,
            (
                box_movement,                  // Handles box movement in the game
                box_ship_collision,            // Detects and processes collisions between the ship and boxes
                ship_movement,                 // Handles ship movement based on user input
                rotate_ship_follow_cursor,     // Rotates the ship to follow the mouse cursor
                shoot_laser,                   // Handles shooting lasers from the ship
                move_laser,                    // Moves the laser in its direction
                detect_laser_collision,        // Detects laser and box collisions
                update_timer_display,          // Updates the game timer display
                detect_starship_box_collision, // Detects collisions between the starship and boxes
                check_end_point_reached,       // Checks if the ship has reached the end point
                animate_fireball,              // Animates the fireball on collision
            ),
        )

        // Start the app
        .run();
}
