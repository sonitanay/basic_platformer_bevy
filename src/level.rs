use crate::physics::*;
use crate::util::*;
use bevy::prelude::*;
pub struct LevelPlugin;

#[derive(Resource)]
pub struct Level {
    pub world_pos: Vec3,
    // level_width: f32,
    // level_height: f32,
    // index: usize,
    pub block_size: Vec2,
    pub grid: Vec<Solid>,
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
        self.grid.push(Solid {
            pos_x: (&self.world_pos.x + &self.block_size.x / 2.) + (&self.block_size.x * tile.0),
            pos_y: (&self.world_pos.y + &self.block_size.y / 2.) + (&self.block_size.y * tile.1),
            texture_file: tile.2,
            bounds: HitBox {
                half_size: BLOCK_SIZE / 2.,
            },
        });
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
        commands.spawn((SpriteBundle {
            texture: asset_server.load(&item.texture_file),
            transform: Transform::from_translation(Vec3::new(
                item.pos_x,
                item.pos_y,
                level.world_pos.z,
            )),
            ..default()
        },));
    }
}
