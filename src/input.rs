use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::component::{Ship, Fireball, BoxEntity, ExplosionTimer, RespawnTimer};

// Ship movement function
pub fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Keyboard inputs for movement
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

        // Adjust ship's position based on input
        let speed = 200.0;
        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();

        // Boundary clamping
        if let Ok(window) = windows.get_single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            let min_x = -half_width + 60.0;
            let max_x = half_width - 60.0;
            let min_y = -half_height + 60.0;
            let max_y = half_height - 60.0;

            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
        }
    }
}

// Rotate ship towards mouse
pub fn rotate_ship_towards_mouse(
    mut query: Query<&mut Transform, With<Ship>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if let Ok(window) = windows.get_single() {
            for event in cursor_moved_events.iter() {
                let ship_position = Vec2::new(transform.translation.x, transform.translation.y);
                let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
                let mouse_position = event.position - window_center;

                let direction = mouse_position - ship_position;
                if direction.length_squared() > 0.0 {
                    let angle = direction.y.atan2(direction.x);
                    transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}

// Collision detection and explosion
pub fn detect_collision_and_explode(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_query: Query<(Entity, &Transform), With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
    mut timer: ResMut<ExplosionTimer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((ship_entity, ship_transform)) = ship_query.get_single_mut() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(box_transform.translation) < collision_distance {
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

                timer.0 = Some(0.5); // Fireball lasts 0.5 seconds
                commands.entity(ship_entity).despawn();

                let reappear_time = 2.0;
                let half_width = windows.get_single().unwrap().width() / 2.0;
                let half_height = windows.get_single().unwrap().height() / 2.0;

                let ship_texture = asset_server.load("ship.png");
                let new_ship = commands.spawn(SpriteBundle {
                    texture: ship_texture,
                    transform: Transform {
                        translation: Vec3::new(-half_width + 40.0, half_height - 40.0, 0.0),
                        scale: Vec3::new(0.1, 0.1, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Ship)
                .id();

                commands.entity(new_ship).insert(RespawnTimer(Timer::from_seconds(reappear_time, TimerMode::Once)));
            }
        }
    }
}

// Despawn fireball after explosion timer ends
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
                commands.entity(entity).despawn();
            }
            timer.0 = None;
        }
    }
}
