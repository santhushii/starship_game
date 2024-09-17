use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        // Add the default plugins (window, rendering, etc.)
        .add_plugins(DefaultPlugins)
        // Run our custom setup system on startup
        .add_startup_system(setup)
        // Add a system for handling keyboard input and moving the ship
        .add_system(ship_movement)
        .run();
}

// Component to identify the ship entity
#[derive(Component)]
struct Ship;

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

        // Spawn the ship at the top-left corner of the screen
        commands.spawn((
            SpriteBundle {
                texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(-half_width + 50.0, half_height - 50.0, 0.0), // Top-left corner with a margin
                    scale: Vec3::new(0.2, 0.2, 1.0), // Smaller scale for the ship
                    ..Default::default()
                },
                ..Default::default()
            },
            Ship, // Add the Ship component to identify this entity
        ));
    }
}

// System to handle ship movement based on keyboard input
fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ship>>, // Query the transform of the ship
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Handle input for movement
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        // Adjust the ship's position based on the input
        let speed = 200.0; // Movement speed (adjust as necessary)
        transform.translation += direction.normalize_or_zero() * speed * TIME_STEP;
    }
}

// Constant for time step to control movement speed per frame
const TIME_STEP: f32 = 1.0 / 60.0;
