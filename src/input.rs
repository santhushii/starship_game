use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::component::{Ship, Fireball, BoxEntity, ExplosionTimer};

// Ship movement function, moves the ship based on keyboard input
pub fn ship_movement(
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
            let min_x = -half_width + 60.0;
            let max_x = half_width - 60.0;
            let min_y = -half_height + 60.0;
            let max_y = half_height - 60.0;

            // Clamp the ship's position to stay within the defined boundaries
            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
        }
    }
}

// Function to detect collisions between the ship and boxes and trigger explosion
pub fn detect_collision_and_explode(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_query: Query<(Entity, &Transform), With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
    time: Res<Time>,
    mut timer: ResMut<ExplosionTimer>,
) {
    if let Ok((ship_entity, ship_transform)) = ship_query.get_single_mut() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(box_transform.translation) < collision_distance {
                // Spawn fireball on collision
                let fireball_texture = asset_server.load("explo_a_sheet.png");
                commands.spawn((
                    SpriteBundle {
                        texture: fireball_texture,
                        transform: Transform {
                            translation: ship_transform.translation,
                            scale: Vec3::new(0.1, 0.1, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Fireball,
                ));

                // Start the explosion timer
                timer.0 = Some(0.5); // Fireball lasts 0.5 seconds

                // Remove the ship entity (simulate explosion)
                commands.entity(ship_entity).despawn();
            }
        }
    }
}

// Function to despawn the fireball after a short time
pub fn fireball_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Fireball)>,
    mut timer: ResMut<ExplosionTimer>,
) {
    if let Some(time_left) = &mut timer.0 {
        *time_left -= time.delta_seconds();
        if *time_left <= 0.0 {
            for (entity, _) in query.iter_mut() {
                // Despawn the fireball after timer runs out
                commands.entity(entity).despawn();
            }
            timer.0 = None; // Reset the timer
        }
    }
}
