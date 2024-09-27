use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::component::{BoxDirection, BoxEntity, EndPoint, GameTimer, Laser, Ship, ShipLives, StartPoint};

// This system sets up the initial game entities, like the ship, start point, end point, and boxes.
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn the 2D camera.
    commands.spawn(Camera2dBundle::default());

    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    // Get the window dimensions
    if let Ok(window) = windows.get_single() {
        let margin = 20.0;
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        // Spawn the starting point (green square).
        let start_point_position = Vec3::new(-half_width + margin, half_height - margin, 0.0);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform {
                translation: start_point_position,
                scale: Vec3::new(20.0, 20.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(StartPoint);

        // Spawn the ship **exactly at the start point**.
        commands.spawn(SpriteBundle {
            texture: ship_handle.clone(),
            transform: Transform {
                translation: start_point_position, // Set the ship at the start point position
                scale: Vec3::new(0.1, 0.1, 1.0),
                rotation: Quat::from_rotation_z(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Ship);

        // Spawn the ending point (red square).
        let end_point_position = Vec3::new(half_width - margin, -half_height + margin, 0.0);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform {
                translation: end_point_position,
                scale: Vec3::new(20.0, 20.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(EndPoint);

        // Spawn 10 moving boxes at random positions with random directions.
        let num_boxes = 10;
        let mut rng = rand::thread_rng();
        let mut box_positions = vec![];

        for _ in 0..num_boxes {
            let mut x;
            let mut y;

            // Avoid overlapping by generating new positions until there's no overlap
            loop {
                x = rng.gen_range(-half_width + margin..half_width - margin);
                y = rng.gen_range(-half_height + margin..half_height - margin);

                let new_position = Vec3::new(x, y, 0.0);
                
                // Ensure no overlap between boxes
                if box_positions.iter().all(|pos: &Vec3| pos.distance(new_position) > 50.0) {
                    box_positions.push(new_position);
                    break;
                }
            }

            let direction = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0,
            ).normalize_or_zero();

            commands.spawn(SpriteBundle {
                texture: box_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: Vec3::new(0.2, 0.2, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(BoxEntity).insert(BoxDirection(direction));
        }

        // Spawn the timer text on the screen
        // Spawn the timer text on the screen in system.rs
commands.spawn(TextBundle {
    text: Text::from_section(
        "Time: 0.0 seconds",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Font path
            font_size: 30.0,  // Make the font smaller for the timer
            color: Color::WHITE,
        }
    ),
    style: Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(20.0),   // Vertical position, 20 pixels from the top
            right: Val::Px(20.0), // Horizontal position, 20 pixels from the right
            ..Default::default()
        },
        ..Default::default()
    },
    ..Default::default()
}).insert(GameTimer(None, false)); // Add the timer component

    }
}

// This system handles box movement and collision detection with boundary clamping.
pub fn box_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BoxDirection), With<BoxEntity>>,
) {
    let speed = 100.0;
    let half_width = 400.0;
    let half_height = 300.0;

    // Collect box data to avoid borrowing issues
    let mut box_data: Vec<(Vec3, Vec3)> = query.iter_mut()
        .map(|(transform, direction)| (transform.translation, direction.0))
        .collect();

    let mut new_positions = vec![];

    // Move boxes and handle boundary detection
    for (translation, direction) in box_data.iter_mut() {
        let movement = *direction * speed * time.delta_seconds();
        *translation += movement;

        // Boundary detection
        if translation.x.abs() > half_width {
            direction.x = -direction.x;
        }
        if translation.y.abs() > half_height {
            direction.y = -direction.y;
        }

        new_positions.push(*translation); // Store the new position
    }

    // Collision detection between boxes
    for i in 0..box_data.len() {
        for j in (i + 1)..box_data.len() {
            if new_positions[i].distance(new_positions[j]) < 40.0 {
                // Swap directions on collision
                let temp = box_data[i].1;
                box_data[i].1 = box_data[j].1;
                box_data[j].1 = temp;
            }
        }
    }

    // Apply changes back to the entities
    for ((mut transform, mut direction), (new_translation, new_direction)) in query.iter_mut().zip(box_data.iter()) {
        transform.translation = *new_translation;
        direction.0 = *new_direction;
    }
}

// Update the visual timer
pub fn update_timer_display(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<&mut Text>,
) {
    let timer_stopped = timer.1;  // Check if the timer is stopped

    if let Some(ref mut elapsed_time) = timer.0 {
        if !timer_stopped {
            *elapsed_time += time.delta_seconds(); // Increment the timer
        }

        // Update the text entity with the elapsed time
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Time: {:.2} seconds", *elapsed_time);
        }
    }
}
pub fn move_laser(
    time: Res<Time>,
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Transform), With<Laser>>,
) {
    let laser_speed = 300.0;

    for (laser_entity, mut transform) in laser_query.iter_mut() {
        transform.translation.y += laser_speed * time.delta_seconds();

        // Despawn laser if it goes out of bounds
        if transform.translation.y > 400.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}
pub fn detect_laser_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    box_query: Query<(Entity, &Transform), With<BoxEntity>>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (box_entity, box_transform) in box_query.iter() {
            let collision_distance = 30.0;
            if laser_transform.translation.distance(box_transform.translation) < collision_distance {
                // Despawn both the laser and the box
                commands.entity(laser_entity).despawn();
                commands.entity(box_entity).despawn();
                break;
            }
        }
    }
}
