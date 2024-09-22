use bevy::prelude::*;

// Ship component to represent the ship entity
#[derive(Component)]
pub struct Ship;

// BoxEntity component to represent the box entity
#[derive(Component)]
pub struct BoxEntity;

// BoxDirection component to store the movement direction of boxes
#[derive(Component)]
pub struct BoxDirection(pub Vec3);

// Fireball component to represent a fireball entity
#[derive(Component)]
pub struct Fireball;

// StartPoint component to represent the start point in the game
#[derive(Component)]
pub struct StartPoint;

// EndPoint component to represent the end point in the game
#[derive(Component)]
pub struct EndPoint;

// Resource to manage fireball's explosion duration
#[derive(Resource)]
pub struct ExplosionTimer(pub Option<f32>);
