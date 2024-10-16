// component.rs
use bevy::prelude::*;

// Starship component
#[derive(Component)]
pub struct Ship;

// Box entity component
#[derive(Component)]
pub struct BoxEntity;

// Box movement direction component
#[derive(Component)]
pub struct BoxDirection(pub Vec3);

// Fireball component (for destruction animation)
#[derive(Component)]
pub struct Fireball;

// Start point component
#[derive(Component)]
pub struct StartPoint;

// End point component
#[derive(Component)]
pub struct EndPoint;

// Game timer resource
#[derive(Resource, Component)]
pub struct GameTimer(pub Option<f32>, pub bool); // Option<f32> for elapsed time, bool to stop the timer

// Starship lives resource
#[derive(Resource, Component)]
pub struct ShipLives(pub u32);

#[derive(Component)]
pub struct Laser {
    #[allow(dead_code)]
    pub laser_type: LaserType, // Suppress warning if not currently used
}

// Enum for different types of lasers
#[derive(Component)]
pub enum LaserType {
    A, // Corresponds to laser type A
    B, // Corresponds to laser type B
}

// Fireball sprite atlas for explosion
#[derive(Resource)]
pub struct FireballAtlas(pub Handle<TextureAtlas>);

// Fireball animation timer
#[derive(Component)]
pub struct FireballAnimationTimer(pub Timer);

// Marker component for lives text (suppress warning if unused)
#[allow(dead_code)]
#[derive(Component)]
pub struct LivesText;

#[allow(dead_code)]
#[derive(Component)]
pub struct ShipLivesDisplay;

#[derive(Component)]
pub struct ScoreDisplay;

// Component to track the player's score
#[derive(Default, Resource)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct LaserMovementTimer(pub Timer);
