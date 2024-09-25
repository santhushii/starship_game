use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::component::{BoxEntity, BoxDirection, Ship, StartPoint, EndPoint};

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

        // Spawn the ship exactly at the start point.
        commands.spawn(SpriteBundle {
            texture: ship_handle.clone(),
            transform: Transform {
                translation: start_point_position, // Start the ship at the same position as start point
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
        for _ in 0..num_boxes {
            let x = rng.gen_range(-half_width + margin..half_width - margin);
            let y = rng.gen_range(-half_height + margin..half_height - margin);

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

    // Collision detection (handled in a separate loop after all movements)
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
