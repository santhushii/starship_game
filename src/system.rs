use bevy::prelude::*;
use crate::component::{
    BoxDirection, BoxEntity, EndPoint, GameTimer, Laser, Ship, StartPoint,
    Fireball, FireballAnimationTimer, ShipLives, FireballAtlas
};
use rand::Rng;
// System to set up initial entities
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn 2D camera
    commands.spawn(Camera2dBundle::default());

    // Load textures
    let ship_handle = asset_server.load("ship.png");
    let box_handle = asset_server.load("box.png");

    // Spawn start point
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-400.0, 300.0, 0.0),
            scale: Vec3::new(20.0, 20.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartPoint);

    // Spawn ship
    commands.spawn(SpriteBundle {
        texture: ship_handle.clone(),
        transform: Transform {
            translation: Vec3::new(-400.0, 300.0, 0.0),
            scale: Vec3::new(0.1, 0.1, 1.0),
            rotation: Quat::from_rotation_z(0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Ship);

    // Spawn end point
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(400.0, -300.0, 0.0),
            scale: Vec3::new(20.0, 20.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(EndPoint);

    // Spawn boxes
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-300.0..300.0);
        let direction = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize_or_zero();

        commands.spawn(SpriteBundle {
            texture: box_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: Vec3::new(0.2, 0.2, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BoxEntity)
        .insert(BoxDirection(direction));
    }

    // Set up fireball sprite atlas
    let fireball_texture_handle = asset_server.load("fireball.png");
    let fireball_atlas = TextureAtlas::from_grid(fireball_texture_handle, Vec2::new(64.0, 64.0), 4, 4, None, None);
    let fireball_atlas_handle = texture_atlases.add(fireball_atlas);
    commands.insert_resource(FireballAtlas(fireball_atlas_handle));

    // Add a timer display text entity
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Time: 0.00 seconds",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

// System to handle box movement (no ship interaction)
pub fn box_movement(
    time: Res<Time>,
    mut box_query: Query<(&mut Transform, &BoxDirection), With<BoxEntity>>,
) {
    let speed = 100.0;

    for (mut box_transform, direction) in box_query.iter_mut() {
        box_transform.translation += direction.0 * speed * time.delta_seconds();

        // Wrap around screen edges
        if box_transform.translation.x > 400.0 || box_transform.translation.x < -400.0 {
            box_transform.translation.x = -box_transform.translation.x;
        }
        if box_transform.translation.y > 300.0 || box_transform.translation.y < -300.0 {
            box_transform.translation.y = -box_transform.translation.y;
        }
    }
}
// System to handle box and ship collision, and spawn fireballs when they collide
pub fn box_ship_collision(
    mut commands: Commands,
    box_query: Query<&Transform, With<BoxEntity>>,
    ship_query: Query<&Transform, With<Ship>>,
    fireball_atlas: Res<FireballAtlas>,
) {
    if let Ok(ship_transform) = ship_query.get_single() {
        for box_transform in box_query.iter() {
            let collision_distance = 30.0; // Adjust this value to match the size of the box and ship
            if box_transform.translation.distance(ship_transform.translation) < collision_distance {
                // Release fireball when a box collides with the ship
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: fireball_atlas.0.clone(),
                    transform: Transform {
                        translation: ship_transform.translation,
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Fireball)
                .insert(FireballAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

                // Additional collision logic can be added here, such as reversing direction
            }
        }
    }
}

// System to move lasers
pub fn move_laser(
    time: Res<Time>,
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Transform), With<Laser>>,
) {
    let laser_speed = 300.0;
    for (laser_entity, mut transform) in laser_query.iter_mut() {
        transform.translation.y += laser_speed * time.delta_seconds();

        // Despawn lasers that go off-screen
        if transform.translation.y > 400.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}

// System to detect laser and box collision
pub fn detect_laser_collision(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    box_query: Query<(Entity, &Transform), With<BoxEntity>>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (box_entity, box_transform) in box_query.iter() {
            let collision_distance = 30.0;
            if laser_transform.translation.distance(box_transform.translation) < collision_distance {
                // Despawn both laser and box
                commands.entity(laser_entity).despawn();
                commands.entity(box_entity).despawn();
                break;
            }
        }
    }
}

// System to animate fireballs
pub fn animate_fireball(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FireballAnimationTimer, &mut TextureAtlasSprite), With<Fireball>>,
) {
    for (entity, mut animation_timer, mut sprite) in query.iter_mut() {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= 16 {
                commands.entity(entity).despawn();
            }
        }
    }
}

// Timer update and display system
pub fn update_timer_display(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<&mut Text>,
) {
    let is_stopped = timer.1;
    if let Some(ref mut elapsed_time) = timer.0 {
        if !is_stopped {
            *elapsed_time += time.delta_seconds();
        }

        // Update the text entity with the elapsed time
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Time: {:.2} seconds", *elapsed_time);
        }
    }
}


// System to detect starship-box collisions
pub fn detect_starship_box_collision(
    mut commands: Commands,
    mut ship_query: Query<(Entity, &Transform, &mut ShipLives), With<Ship>>,
    box_query: Query<&Transform, With<BoxEntity>>,
    fireball_atlas: Res<FireballAtlas>,
    asset_server: Res<AssetServer>,
) {
    // Check if the ship exists and handle ship logic
    if let Ok((ship_entity, ship_transform, mut lives)) = ship_query.get_single_mut() {
        let ship_position = ship_transform.translation;

        // Now we handle the box collision logic in a separate query
        let mut collided = false;
        for box_transform in box_query.iter() {
            let collision_distance = 30.0;
            if ship_position.distance(box_transform.translation) < collision_distance {
                collided = true;

                // Spawn fireball at the collision point
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: fireball_atlas.0.clone(),
                    transform: Transform {
                        translation: ship_position,
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Fireball)
                .insert(FireballAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

                break;
            }
        }

        if collided {
            // Reduce ship lives
            lives.0 -= 1;
            println!("Lives remaining: {}", lives.0);

            // If no lives left, despawn the ship and display game over message
            if lives.0 == 0 {
                println!("Game Over!");
                commands.entity(ship_entity).despawn();

                commands.spawn(TextBundle {
                    text: Text::from_section(
                        "Game Over",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::RED,
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(250.0),
                        left: Val::Px(200.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            } else {
                // Reset ship's position to start point if lives remain
                commands.entity(ship_entity).insert(Transform {
                    translation: Vec3::new(-400.0, 300.0, 0.0),
                    ..Default::default()
                });
            }
        }
    }
}

// System to check if the ship has reached the end point
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
                        "Level Complete",
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::GREEN,
                        }
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(100.0),
                        left: Val::Px(200.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}
