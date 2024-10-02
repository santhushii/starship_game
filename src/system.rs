use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::component::{BoxDirection, BoxEntity, EndPoint, GameTimer, Laser, Ship, StartPoint, Fireball, FireballAtlas, FireballAnimationTimer};
//use crate::component::GameTimer;

// Set up initial entities
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    // Get window dimensions
    if let Ok(window) = windows.get_single() {
        let margin = 20.0;
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;

        // Spawn start point
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

        // Spawn ship
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

        // Spawn end point
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

        // Spawn boxes
        let num_boxes = 10;
        let mut rng = rand::thread_rng();
        let mut box_positions = vec![];

        for _ in 0..num_boxes {
            let mut x;
            let mut y;

            // Avoid overlapping
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

        // Spawn the timer text on the screen
        commands.spawn(TextBundle {
            text: Text::from_section(
                "Time: 0.0 seconds",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                }
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }).insert(GameTimer(None, false));

        // Set up fireball sprite atlas
        let fireball_texture_handle = asset_server.load("fireball.png");
        let fireball_atlas = TextureAtlas::from_grid(
            fireball_texture_handle,
            Vec2::new(64.0, 64.0), // size of each frame
            4, 4, // number of columns and rows
            None, None // no padding between sprites
        );
        
        let fireball_atlas_handle = texture_atlases.add(fireball_atlas);

        commands.insert_resource(FireballAtlas(fireball_atlas_handle));
    }
}

// Box movement system
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

        new_positions.push(*translation);
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

// Move laser system
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

// Detect laser-box collision system
pub fn detect_laser_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    box_query: Query<(Entity, &Transform), With<BoxEntity>>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (box_entity, box_transform) in box_query.iter() {
            let collision_distance = 30.0;
            if laser_transform.translation.distance(box_transform.translation) < collision_distance {
                commands.entity(laser_entity).despawn();
                commands.entity(box_entity).despawn();
                break;
            }
        }
    }
}

// Animate fireball system
pub fn animate_fireball(
    time: Res<Time>,
    mut commands: Commands,
    mut fireball_query: Query<(Entity, &mut FireballAnimationTimer, &mut TextureAtlasSprite), With<Fireball>>,
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