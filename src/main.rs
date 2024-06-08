use bevy::prelude::*;
use level::{Level, LevelPlugin};
mod level;
mod player;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(
            Level::new()
                .repeat((0., 0., "tile_0069.png".to_string()), 79., 0.)
                .repeat((0., 1., "tile_0069.png".to_string()), 0., 43.),
        )
        //.repeat((0., 0., "tile_0069.png".to_string()), 80., 1.)
        //.add_tile((1., 0., "tile_0069.png".to_string()))
        .add_plugins(LevelPlugin)
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(640., 360., 0.)),
        ..default()
    });
}
