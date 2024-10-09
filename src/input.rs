use bevy::prelude::*;
use crate::component::{EndPoint, GameTimer, Laser, LaserType, Ship, StartPoint};

// Add a resource to track which laser type to shoot
#[derive(Resource, Default)]
pub struct LaserTypeTracker {
    pub shoot_a: bool, // True if `laser_a_01.png` is to be shot, False for `laser_b_01.png`
}

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
    mut laser_tracker: ResMut<LaserTypeTracker>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(ship_transform) = ship_query.get_single() {
            // Alternate between laser_a_01.png and laser_b_01.png
            let laser_texture = if laser_tracker.shoot_a {
                asset_server.load("laser_a_01.png")
            } else {
                asset_server.load("laser_b_01.png")
            };
            let laser_position = ship_transform.translation;

            commands.spawn(SpriteBundle {
                texture: laser_texture,
                transform: Transform {
                    translation: laser_position,
                    scale: Vec3::new(0.3, 0.3, 1.0), // Adjusted laser size
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser {
                laser_type: if laser_tracker.shoot_a { LaserType::A } else { LaserType::B },
            });

            // Toggle the laser type for the next shot
            laser_tracker.shoot_a = !laser_tracker.shoot_a;
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
top: Val::Px(250.0),
left: Val::Px(200.0),
..Default::default()

                    },
                    ..Default::default()
                });
            }
        }
    }
}
