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

// Resource for managing explosion duration
#[derive(Resource)]
pub struct ExplosionTimer(pub Option<f32>);

// RespawnTimer component for ship respawn after explosion
#[derive(Component)]
pub struct RespawnTimer(pub Timer);
