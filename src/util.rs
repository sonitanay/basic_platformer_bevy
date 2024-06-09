use bevy::prelude::*;
pub const BLOCK_SIZE: Vec2 = Vec2::splat(16.);
pub const GRAVITY_DEFAULT: f32 = 320.;
pub const JMP_VEL_PLAYER: f32 = 160.;
pub const INITIAL_ACCEL_PLAYER: f32 = 1200.;
pub const MAX_VEL_PLAYER: f32 = 240.;
pub const MIN_VEL_PLAYER: f32 = 4.;
pub const FRICTION: f32 = 150.;
pub const INITIAL_VEL_DASH: f32 = 880.;
pub const INITIAL_ACCEL_DASH: f32 = 3800.;
pub const DEFAULT_DASH_COUNT: usize = 2;
pub const DEFAULT_DASH_DURATION: f32 = 0.2;

#[derive(Component)]
pub struct HitBox {
    pub half_size: Vec2,
}

#[derive(Component)]
pub struct CameraMarker;
