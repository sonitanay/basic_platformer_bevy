use bevy::{prelude::*, render::camera::ScalingMode};
use level::{Level, LevelPlugin};
use physics::PhysicsPlugin;
use player::{PlayerMarker, PlayerPlugin};
use util::CameraMarker;
mod level;
mod physics;
mod player;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(
            Level::new()
                .repeat((0., 0., "tile_0069.png".to_string()), 30., 0.)
                .repeat((0., 1., "tile_0069.png".to_string()), 0., 43.),
        )
        //.repeat((0., 0., "tile_0069.png".to_string()), 80., 1.)
        //.add_tile((1., 0., "tile_0069.png".to_string()))
        .add_plugins(LevelPlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, update_camera)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_world(mut commands: Commands) {
    let mut cam = Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(640., 360., 0.)),
        ..default()
    };
    cam.projection.scaling_mode = ScalingMode::AutoMax {
        max_width: 352.,
        max_height: 198.,
    };

    commands.spawn((cam, CameraMarker));
}
fn update_camera(
    mut query_cam: Query<&mut Transform, With<CameraMarker>>,
    query_player: Query<&Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
) {
    let mut cam = query_cam.single_mut();
    let player = query_player.single();
    cam.translation = player.translation;
}
