use bevy::prelude::*;
use crate::component::{BoxEntity, EndPoint, GameTimer, Laser, Ship, ShipLives, StartPoint};
use crate::component::Fireball;


// Ship movement function with arrow keys
pub fn ship_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Ship>>,
        Query<&Transform, With<StartPoint>>,
    )>,
    time: Res<Time>,
    windows: Query<&Window>, // Access the window dimensions using Query
    mut timer: ResMut<GameTimer>,
) {
    if let Ok(mut transform) = param_set.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;

        // Start the game timer when the player presses a movement key
        if timer.0.is_none() 
            && (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)
            || keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)
            || keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) 
        {
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

        // Get the primary window to dynamically determine the screen dimensions
        if let Ok(window) = windows.get_single() {
            let half_width = window.width() / 2.0;
            let half_height = window.height() / 2.0;

            // Set boundary clamping to allow the ship to move across the entire window
            let min_x = -half_width;
            let max_x = half_width;
            let min_y = -half_height;
            let max_y = half_height;

            // Clamp the starship's position within the screen bounds
            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
            transform.translation.y = transform.translation.y.clamp(min_y, max_y);
        }
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
            let new_rotation = current_rotation + std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(new_rotation);
        }
    }
}

// Detect collision between ship and boxes, spawn fireball
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
                    commands.spawn(TextBundle {
                        text: Text::from_section(
                            "Game Over",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::RED,
                            }
                        ),
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(250.0),
                                left: Val::Px(200.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    return;
                }

                // Respawn ship at start point
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

                // Spawn fireball at collision location
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

// Shoot laser system
pub fn shoot_laser(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    ship_query: Query<&Transform, With<Ship>>,
    asset_server: Res<AssetServer>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
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

// Check if ship reaches end point
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
                commands.entity(ship_entity).despawn();
                timer.1 = true;

                commands.spawn(TextBundle {
                    text: Text::from_section(
                        "Level One Complete",
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
