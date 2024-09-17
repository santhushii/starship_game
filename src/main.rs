use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        // Add the default plugins (window, rendering, etc.)
        .add_plugins(DefaultPlugins)
        // Run our custom setup system on startup
        .add_startup_system(setup)
        .run();
}

// The setup system that runs when the app starts
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>, // Query the primary window
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Load the ship texture (make sure "assets/ship.png" exists)
    let ship_handle = asset_server.load("ship.png");

    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        // Move the ship to the top-left corner of the screen
        commands.spawn(SpriteBundle {
            texture: ship_handle,
            transform: Transform {
                translation: Vec3::new(-half_width + 50.0, half_height - 50.0, 0.0), // Top-left corner with a margin
                scale: Vec3::new(0.2, 0.2, 1.0), // Smaller scale for the ship
                ..Default::default()
            },
            ..Default::default()
        });
    } else {
        // Handle the case where the window is not found (this shouldn't happen in normal circumstances)
        eprintln!("Primary window not found");
    }
}
