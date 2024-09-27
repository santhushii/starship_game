use bevy::prelude::*;

// Ship component
#[derive(Component)]
pub struct Ship;

// BoxEntity component
#[derive(Component)]
pub struct BoxEntity;

// BoxDirection component for box movement direction
#[derive(Component)]
pub struct BoxDirection(pub Vec3);

// Fireball component
#[derive(Component)]
pub struct Fireball;

// StartPoint component for game start point
#[derive(Component)]
pub struct StartPoint;

// EndPoint component for game end point
#[derive(Component)]
pub struct EndPoint;

// Game timer resource
#[derive(Resource, Component)]
pub struct GameTimer(pub Option<f32>, pub bool); // Option<f32>: elapsed time, bool: stop timer

// Lives resource for tracking starship's lives
#[derive(Resource, Component)]
pub struct ShipLives(pub u32); // Track the number of remaining lives

#[derive(Component)]
pub struct Laser;

#[derive(Resource)]
pub struct FireballAtlas(Handle<TextureAtlas>);

