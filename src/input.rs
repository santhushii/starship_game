use bevy::prelude::*;
use crate::component::{BoxEntity, EndPoint, GameTimer, Laser, Ship, ShipLives, StartPoint, Fireball};

// 1. **Ship Movement and Rotation:**
pub fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Ship>>,
        Query<&Transform, With<StartPoint>>,
    )>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut timer: ResMut<GameTimer>,
) {
    if let Ok(mut transform) = param_set.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Start the game timer when the player moves
        if timer.0.is_none() && (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Down)
            || keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S)
            || keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D)) {
            timer.0 = Some(0.0);
        }

        // Arrow keys or WASD movement
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

        let speed = 200.0;
        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();

        if let Ok(window) = windows.get_single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
            transform.translation.y = transform.translation.y.clamp(-half_height, half_height);
        }
    }
}

// **2. Rotate ship based on mouse click:**
pub fn rotate_ship_on_click(
    mut ship_query: Query<&mut Transform, With<Ship>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Ok(mut transform) = ship_query.get_single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            // Rotate 90 degrees clockwise
            let current_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
            let new_rotation = current_rotation + std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(new_rotation);
        }
    }
}

// **3. Shooting laser system (Spacebar):**
pub fn shoot_laser(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    ship_query: Query<&Transform, With<Ship>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(ship_transform) = ship_query.get_single() {
            let laser_texture = asset_server.load("laser_a_01.png");
            let laser_position = ship_transform.translation;

            commands.spawn(SpriteBundle {
                texture: laser_texture,
                transform: Transform {
                    translation: laser_position,
                    scale: Vec3::new(0.1, 0.1, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Laser);
        }
    }
}

// **4. End Point Check - Game Ends:**
pub fn check_end_point_reached(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Ship>>,
    end_point_query: Query<&Transform, With<EndPoint>>,
    mut timer: ResMut<GameTimer>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((ship_entity, ship_transform)) = query.get_single_mut() {
        if let Ok(end_point_transform) = end_point_query.get_single() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(end_point_transform.translation) < collision_distance {
                println!("Ship reached the end point!");
                commands.entity(ship_entity).despawn(); // Remove the starship
                timer.1 = true;

                // Display "Game Over" message
                commands.spawn(TextBundle {
                    text: Text::from_section(
                        "Game Over!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::GREEN,
                        }
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(100.0),
                            right: Val::Px(20.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}

// **5. Detect collision between ship and box, spawn fireballs:**
pub fn detect_collision_and_spawn_fireballs(
    mut commands: Commands,
    mut ship_query: Query<(Entity, &Transform), With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
    start_point_query: Query<&Transform, With<StartPoint>>,
    asset_server: Res<AssetServer>,
    mut lives: ResMut<ShipLives>,
) {
    if let Ok((ship_entity, ship_transform)) = ship_query.get_single_mut() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_transform.translation.distance(box_transform.translation) < collision_distance {
                lives.0 -= 1;
                println!("Lives left: {}", lives.0);

                if lives.0 == 0 {
                    println!("Game Over! No lives left.");
                    commands.entity(ship_entity).despawn();
                    return;
                }

                // Respawn the ship at the start point
                commands.entity(ship_entity).despawn();

                if let Ok(start_transform) = start_point_query.get_single() {
                    let start_position = start_transform.translation;
                    let ship_texture = asset_server.load("ship.png");

                    commands.spawn(SpriteBundle {
                        texture: ship_texture,
                        transform: Transform {
                            translation: start_position,
                            scale: Vec3::new(0.1, 0.1, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Ship);
                }

                // Spawn fireball after collision
                let fireball_texture = asset_server.load("fireball.png");
                commands.spawn(SpriteBundle {
                    texture: fireball_texture,
                    transform: Transform {
                        translation: ship_transform.translation,
                        scale: Vec3::new(0.1, 0.1, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Fireball);
            }
        }
    }
}
