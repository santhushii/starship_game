use bevy::prelude::*;
use crate::component::{Ship, BoxEntity, StartPoint, EndPoint, GameTimer, Fireball};

// Ship movement function with arrow keys and WASD support
pub fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Ship>>,
        Query<&Transform, With<StartPoint>>,
    )>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
) {
    if let Ok(mut transform) = param_set.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Start the game timer when the player presses a movement key
        if timer.0.is_none() && (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) ||
            keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) ||
            keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) ||
            keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) {
            timer.0 = Some(0.0);
        }

        // Check for arrow keys and WASD keys for movement
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        // Adjust ship's position based on input
        let speed = 200.0;
        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();

        // Boundary clamping to keep the ship within the window
        let half_width = 400.0; // Assume fixed window width
        let half_height = 300.0; // Assume fixed window height

        let min_x = -half_width + 60.0;
        let max_x = half_width - 60.0;
        let min_y = -half_height + 60.0;
        let max_y = half_height - 60.0;

        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}

// Rotate ship in 4 directions based on mouse click
pub fn rotate_ship_on_click(
    mut ship_query: Query<&mut Transform, With<Ship>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Ok(mut transform) = ship_query.get_single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

            // Rotate the ship by 90 degrees (FRAC_PI_2 is π/2, or 90 degrees in radians)
            let new_rotation = current_rotation + std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(new_rotation);
        }
    }
}

// End the game when the ship reaches the end point
pub fn check_end_point_reached(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Ship>>,
    end_point_query: Query<&Transform, With<EndPoint>>,
    mut timer: ResMut<GameTimer>,
) {
    if let Ok((ship_entity, ship_transform)) = query.get_single_mut() {
        if let Ok(end_point_transform) = end_point_query.get_single() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(end_point_transform.translation) < collision_distance {
                commands.entity(ship_entity).despawn(); // Despawn the ship
                timer.1 = true; // Stop the timer
                println!("Game Over! Reached the End Point.");
            }
        }
    }
}

// Track and display the game timer
pub fn track_game_timer(time: Res<Time>, mut timer: ResMut<GameTimer>) {
    let timer_stopped = timer.1; // Make a copy of timer.1 (whether the timer is stopped)

    if let Some(ref mut elapsed_time) = timer.0 {
        if !timer_stopped {
            *elapsed_time += time.delta_seconds();
            println!("Game Timer: {:.2} seconds", *elapsed_time);
        }
    }
}

// Collision detection, reset ship at start point, and spawn fireballs
// Collision detection and reset ship at start point
pub fn detect_collision_and_spawn_fireballs(
    mut commands: Commands,
    mut ship_query: Query<(Entity, &Transform), With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
    start_point_query: Query<&Transform, With<StartPoint>>, // Start point position for respawn
    asset_server: Res<AssetServer>,
) {
    if let Ok((ship_entity, ship_transform)) = ship_query.get_single_mut() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(box_transform.translation) < collision_distance {
                // Despawn the current ship and reset it at the starting point
                commands.entity(ship_entity).despawn();

                if let Ok(start_transform) = start_point_query.get_single() {
                    let start_position = start_transform.translation; // Fetch start point position
                    let ship_texture = asset_server.load("ship.png");

                    // Respawn the ship at the start point
                    commands.spawn(SpriteBundle {
                        texture: ship_texture,
                        transform: Transform {
                            translation: start_position, // Reset to the start point position
                            scale: Vec3::new(0.1, 0.1, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Ship);
                }

                // Spawn a fireball at the collision location
                let fireball_texture = asset_server.load("fireball.png");
                commands.spawn(SpriteBundle {
                    texture: fireball_texture,
                    transform: Transform {
                        translation: ship_transform.translation, // Spawn fireball where the collision happens
                        scale: Vec3::new(0.1, 0.1, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Fireball);
            }
        }
    }
}
