use bevy::prelude::*;

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
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Load the ship texture (make sure "assets/ship.png" exists)
    let ship_handle = asset_server.load("ship.png");

    // Spawn the ship sprite at the center of the screen
    commands.spawn(SpriteBundle {
        texture: ship_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // Ship position
            scale: Vec3::new(0.5, 0.5, 1.0), // Adjust the scale of the ship
            ..Default::default()
        },
        ..Default::default()
    });
}
