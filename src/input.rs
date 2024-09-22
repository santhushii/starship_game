use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::component::{Ship, Fireball, BoxEntity};

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

pub fn ship_rotation(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Ship>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(mouse_pos) = window.cursor_position() {
                if let Ok(mut transform) = query.get_single_mut() {
                    let window_size = Vec2::new(window.width(), window.height());
                    let mouse_world_pos = Vec3::new(
                        (mouse_pos.x - window_size.x / 2.0) / (window_size.x / 2.0),
                        (mouse_pos.y - window_size.y / 2.0) / (window_size.y / 2.0),
                        0.0,
                    );

                    let direction = (mouse_world_pos - transform.translation).truncate();
                    let angle = direction.y.atan2(direction.x);

                    transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}

pub fn spawn_fireball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ship_query: Query<&Transform, With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
) {
    for ship_transform in ship_query.iter() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_transform
                .translation
                .distance(box_transform.translation) < collision_distance
            {
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
            }
        }
    }
}
