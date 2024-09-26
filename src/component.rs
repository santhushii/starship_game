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

// Explosion timer resource
#[derive(Resource)]
pub struct ExplosionTimer(pub Option<f32>);

// Respawn timer component for ship respawn after explosion
#[derive(Component)]
pub struct RespawnTimer(pub Timer);

// Game timer resource
#[derive(Resource, Component)]  // Add `Component` to allow usage in Query
pub struct GameTimer(pub Option<f32>, pub bool); // Option<f32>: elapsed time, bool: stop timer
