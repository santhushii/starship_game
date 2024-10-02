use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::component::{BoxDirection, BoxEntity, EndPoint, Laser, Ship, StartPoint, FireballAnimationTimer};
use rand::Rng;

// **1. Setup entities:**
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

        // Start Point
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

        // Starship
        commands.spawn(SpriteBundle {
            texture: ship_handle.clone(),
            transform: Transform {
                translation: start_point_position,
                scale: Vec3::new(0.1, 0.1, 1.0),
                rotation: Quat::from_rotation_z(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Ship);

        // End Point
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

        // Boxes
        let num_boxes = 10;
        let mut rng = rand::thread_rng();
        let mut box_positions = vec![];

        for _ in 0..num_boxes {
            let mut x;
            let mut y;

            loop {
                x = rng.gen_range(-half_width + margin..half_width - margin);
                y = rng.gen_range(-half_height + margin..half_height - margin);

                let new_position = Vec3::new(x, y, 0.0);
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
    }
}

// **2. Box movement system:**
pub fn box_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BoxDirection), With<BoxEntity>>,
) {
    let speed = 100.0;
    for (mut transform, mut direction) in query.iter_mut() {
        transform.translation += direction.0 * speed * time.delta_seconds();

        // Bounce off the screen edges
        if transform.translation.x > 400.0 || transform.translation.x < -400.0 {
            direction.0.x = -direction.0.x;
        }
        if transform.translation.y > 300.0 || transform.translation.y < -300.0 {
            direction.0.y = -direction.0.y;
        }
    }
}

// **3. Move laser system:**
pub fn move_laser(
    time: Res<Time>,
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Transform), With<Laser>>,
) {
    let laser_speed = 300.0;
    for (laser_entity, mut transform) in laser_query.iter_mut() {
        transform.translation.y += laser_speed * time.delta_seconds();

        // Despawn laser if out of bounds
        if transform.translation.y > 400.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}

// **4. Detect laser-box collision system and handle box destruction:**
pub fn detect_laser_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &Laser), With<Laser>>,
    box_query: Query<(Entity, &Transform), With<BoxEntity>>,
    asset_server: Res<AssetServer>,
) {
    for (laser_entity, laser_transform, _laser) in laser_query.iter() {
        for (box_entity, box_transform) in box_query.iter() {
            let collision_distance = 30.0;
            if laser_transform.translation.distance(box_transform.translation) < collision_distance {
                // Destroy both laser and box
                commands.entity(laser_entity).despawn();
                commands.entity(box_entity).despawn();

                // Play box destruction animation (explosion)
                let explosion_texture = asset_server.load("fireball.png");
                commands.spawn(SpriteBundle {
                    texture: explosion_texture,
                    transform: Transform {
                        translation: box_transform.translation,
                        scale: Vec3::new(0.5, 0.5, 1.0), // Adjusted explosion size
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoxEntity) // Reuse BoxEntity for the animation
                .insert(FireballAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

                break; // Exit loop after one collision
            }
        }
    }
}

// **5. Animate Fireball System:**
pub fn animate_fireball(
    time: Res<Time>,
    mut commands: Commands,
    mut fireball_query: Query<(Entity, &mut FireballAnimationTimer, &mut TextureAtlasSprite), With<BoxEntity>>,
) {
    for (entity, mut animation_timer, mut sprite) in fireball_query.iter_mut() {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= 16 {
                commands.entity(entity).despawn(); // Despawn fireball after animation completes
            }
        }
    }
}
