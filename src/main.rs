use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng; // Import the rand crate for random number generation

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(ship_movement)
        .add_system(ship_rotation)
        .run();
}

// Component to identify the ship entity
#[derive(Component)]
struct Ship;

// Component to identify start and end points
#[derive(Component)]
struct StartPoint;
#[derive(Component)]
struct EndPoint;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Load the ship and box textures
    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let margin = 20.0; // Define a margin from the window edges
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        // Spawn the start point visual (e.g., a green square)
        commands.spawn(( 
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(-half_width + margin, half_height - margin, 0.0), // Top-left corner with margin
                    scale: Vec3::new(20.0, 20.0, 1.0), // Larger size for the start point visual
                    ..Default::default()
                },
                ..Default::default()
            },
            StartPoint,
        ));

        // Spawn the end point visual (e.g., a red square)
        commands.spawn(( 
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(half_width - margin, -half_height + margin, 0.0), // Bottom-right corner with margin
                    scale: Vec3::new(20.0, 20.0, 1.0), // Larger size for the end point visual
                    ..Default::default()
                },
                ..Default::default()
            },
            EndPoint,
        ));

        // Spawn the ship near the start point (right side)
        commands.spawn(( 
            SpriteBundle {
                texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(-half_width + margin + 40.0, half_height - margin - 40.0, 0.0), // Right side of the top-left corner
                    scale: Vec3::new(0.1, 0.1, 1.0), // Smaller size for the ship
                    rotation: Quat::from_rotation_z(0.0), // Initial rotation
                    ..Default::default()
                },
                ..Default::default()
            },
            Ship, // Add the Ship component to identify this entity
        ));

        // Spawn 10 boxes at random positions
        let num_boxes = 10;
        let mut rng = rand::thread_rng(); // Create a random number generator
        for _ in 0..num_boxes {
            let x = rng.gen_range(-half_width + margin..half_width - margin);
            let y = rng.gen_range(-half_height + margin..half_height - margin);

            commands.spawn(( 
                SpriteBundle {
                    texture: box_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(0.1, 0.1, 1.0), // Adjust size as needed
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Optionally add a Box component if needed
            ));
        }
    }
}

// System to handle ship movement based on keyboard input
fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
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
        let speed = 200.0;
        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();

        // Get window size for boundary clamping
        if let Ok(window) = windows.get_single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            // Define the margin boundaries
            let min_x = -half_width + 60.0; // Start point margin (left side)
            let max_x = half_width - 60.0;  // End point margin (right side)
            let min_y = -half_height + 60.0; // End point margin (bottom side)
            let max_y = half_height - 60.0;  // Start point margin (top side)

            // Clamp the ship's position to stay within the defined boundaries
            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
        }
    }
}

// System to handle ship rotation based on mouse click
fn ship_rotation(
    mut mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Ship>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(mouse_pos) = window.cursor_position() {
                if let Ok(mut transform) = query.get_single_mut() {
                    // Convert the mouse position to world coordinates
                    let window_size = Vec2::new(window.width(), window.height());
                    let mouse_world_pos = Vec3::new(
                        (mouse_pos.x - window_size.x / 2.0) / (window_size.x / 2.0),
                        (mouse_pos.y - window_size.y / 2.0) / (window_size.y / 2.0),
                        0.0,
                    );

                    // Calculate the direction vector and angle to rotate
                    let direction = (mouse_world_pos - transform.translation).truncate();
                    let angle = direction.y.atan2(direction.x);

                    // Update the ship's rotation
                    transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}
