use bevy::prelude::*;

// Component to identify the ship entity
#[derive(Component)]
pub struct Ship;

// Component to identify start and end points
#[derive(Component)]
pub struct StartPoint;
#[derive(Component)]
pub struct EndPoint;

// Component to identify the box entities
#[derive(Component)]
pub struct BoxEntity;

// Component to store the direction of the box movement
#[derive(Component)]
pub struct BoxDirection(pub Vec3);
