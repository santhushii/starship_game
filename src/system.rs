use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::component::{BoxEntity, BoxDirection, Ship, StartPoint, EndPoint};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    if let Ok(window) = windows.get_single() {
        let margin = 20.0;
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(-half_width + margin, half_height - margin, 0.0),
                    scale: Vec3::new(20.0, 20.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            StartPoint,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(half_width - margin, -half_height + margin, 0.0),
                    scale: Vec3::new(20.0, 20.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            EndPoint,
        ));

        commands.spawn((
            SpriteBundle {
                texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(-half_width + margin + 40.0, half_height - margin - 40.0, 0.0),
                    scale: Vec3::new(0.1, 0.1, 1.0),
                    rotation: Quat::from_rotation_z(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Ship,
        ));

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

            commands.spawn((
                SpriteBundle {
                    texture: box_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: Vec3::new(0.2, 0.2, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                BoxEntity,
                BoxDirection(direction),
            ));
        }
    }
}

// Box movement with collision detection and reversal of direction
pub fn box_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BoxDirection), With<BoxEntity>>,
) {
    let speed = 100.0;

    // Iterate over each box and move it
    for (mut transform, mut direction) in query.iter_mut() {
        let movement = direction.0 * speed * time.delta_seconds();
        transform.translation += movement;

        let half_width = 400.0;
        let half_height = 300.0;

        if transform.translation.x.abs() > half_width {
            direction.0.x = -direction.0.x;
        }
        if transform.translation.y.abs() > half_height {
            direction.0.y = -direction.0.y;
        }
    }

    // Nested loop for collision detection
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut transform_a, mut direction_a), (mut transform_b, mut direction_b)]) = combinations.fetch_next() {
        if transform_a.translation.distance(transform_b.translation) < 40.0 {
            // Reverse directions on collision
            let temp = direction_a.0;
            direction_a.0 = direction_b.0;
            direction_b.0 = temp;
        }
    }
}
