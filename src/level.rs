use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use crate::physics::*;
use crate::util::*;
use bevy::prelude::*;
use serde_json::*;
pub struct LevelPlugin;

#[derive(Resource)]
pub struct Level {
    pub world_pos: Vec3,
    // level_width: f32,
    // level_height: f32,
    // index: usize,
    pub block_size: Vec2,
    pub grid: Vec<Solid>,
    pub player_spawn_pos: Vec2,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_level_init);
    }
}

impl Level {
    // pub fn new() -> Level {
    //     Level {
    //         world_pos: Vec3::new(0., 0., 0.),
    //         // level_width: 1280.,
    //         // level_height: 720.,
    //         // index: 0,
    //         block_size: BLOCK_SIZE,
    //         grid: Vec::new(),
    //         player_spawn_pos: Vec2::ZERO,
    //     }
    // }

    pub fn new_from_json(filepath: String) -> Level {
        let fs: File = File::open(filepath).unwrap();
        let mut reader = BufReader::new(fs);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        let value: Value = serde_json::from_str(buffer.as_str()).unwrap();

        let world_height = value["defaultLevelWidth"].as_f64().unwrap() as f32;

        let spawn_pos_x = value["levels"][0]["layerInstances"][0]["entityInstances"][0]["px"][0]
            .as_f64()
            .unwrap() as f32;
        let spawn_pos_y = value["levels"][0]["layerInstances"][0]["entityInstances"][0]["px"][1]
            .as_f64()
            .unwrap() as f32;

        let mut lvl = Level {
            world_pos: Vec3::new(0., 0., 0.),
            block_size: BLOCK_SIZE,
            grid: Vec::new(),
            player_spawn_pos: Vec2::new(spawn_pos_x, world_height - spawn_pos_y),
        };
        let blocks = value["levels"][0]["layerInstances"][1]["gridTiles"]
            .as_array()
            .unwrap();
        for item in blocks {
            let t_x: f32 = item["px"][0].as_f64().unwrap() as f32;
            let t_y: f32 = item["px"][1].as_f64().unwrap() as f32;

            // pos_x: t_x / BLOCK_SIZE.x,
            //     pos_y: t_y / BLOCK_SIZE.y,
            lvl.add_tile((
                t_x / BLOCK_SIZE.x,
                (world_height - t_y) / BLOCK_SIZE.y,
                "tile_0069.png".to_string(),
            ));
        }
        lvl
    }

    pub fn add_tile(&mut self, tile: (f32, f32, String)) -> &mut Level {
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

    // pub fn repeat(&mut self, tile: (f32, f32, String), repeat_x: f32, repeat_y: f32) -> &mut Level {
    //     let mut ix = tile.0;
    //     let mut iy = tile.1;

    //     while iy <= (repeat_y + tile.1) {
    //         while ix <= (repeat_x + tile.0) {
    //             self.add_tile((ix, iy, tile.2.to_owned()));
    //             ix += 1.
    //         }
    //         ix = tile.0;
    //         iy += 1.;
    //     }
    //     self
    // }
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
