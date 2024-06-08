use bevy::prelude::*;
pub const BLOCK_SIZE: Vec2 = Vec2::splat(16.);

#[derive(Component)]
pub struct HitBox {
    pub half_size: Vec2,
}
