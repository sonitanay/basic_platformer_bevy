use crate::util::*;
use bevy::prelude::*;
pub struct LevelPlugin;

#[derive(Resource)]
pub struct Level {
    world_pos: Vec3,
    // level_width: f32,
    // level_height: f32,
    // index: usize,
    block_size: Vec2,
    grid: Vec<(f32, f32, String)>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_level_init);
    }
}

impl Level {
    pub fn new() -> Level {
        Level {
            world_pos: Vec3::new(0., 0., 0.),
            // level_width: 1280.,
            // level_height: 720.,
            // index: 0,
            block_size: BLOCK_SIZE,
            grid: Vec::new(),
        }
    }

    pub fn add_tile(mut self, tile: (f32, f32, String)) -> Level {
        self.grid.push(tile);
        self
    }

    pub fn repeat(mut self, tile: (f32, f32, String), repeat_x: f32, repeat_y: f32) -> Level {
        let mut ix = tile.0;
        let mut iy = tile.1;

        while iy <= (repeat_y + tile.1) {
            while ix <= (repeat_x + tile.0) {
                self = self.add_tile((ix, iy, tile.2.to_owned()));
                ix += 1.
            }
            ix = tile.0;
            iy += 1.;
        }
        self
    }
}

fn startup_level_init(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    for item in &level.grid {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&item.2),
                transform: Transform::from_translation(Vec3::new(
                    (&level.world_pos.x + &level.block_size.x / 2.)
                        + (&level.block_size.x * item.0),
                    (&level.world_pos.y + &level.block_size.y / 2.)
                        + (&level.block_size.y * item.1),
                    level.world_pos.z,
                )),
                ..default()
            },
            Solid,
            HitBox {
                half_size: BLOCK_SIZE / 2.,
            },
        ));
    }
}

#[derive(Component)]
pub struct Solid;
