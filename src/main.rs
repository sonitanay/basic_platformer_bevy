use bevy::{prelude::*, render::camera::ScalingMode};
use level::{Level, LevelPlugin};
use physics::{Movement, PhysicsPlugin};
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
        // .insert_resource(
        //     Level::new()
        //         .repeat((0., 0., "tile_0069.png".to_string()), 79., 0.)
        //         .repeat((0., 1., "tile_0069.png".to_string()), 0., 42.)
        //         .repeat((20., 10., "tile_0069.png".to_string()), 39., 0.)
        //         .repeat((79., 1., "tile_0069.png".to_string()), 0., 42.)
        //         .repeat((0., 44., "tile_0069.png".to_string()), 79., 0.),
        // )
        .insert_resource(Level::new_from_json(
            "W:\\Rust Projects\\basic_platformer\\assets\\level_1.json".to_string(),
        ))
        .add_plugins(LevelPlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, (update_camera, draw_dash_distance))
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
        max_width: 640.,
        max_height: 360.,
    };

    let text_style = TextStyle {
        font_size: 20.,
        ..default()
    };

    commands.spawn((cam, CameraMarker));
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new("dash_distance : ", text_style.clone()),
            TextSection::new("", text_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        }),
    );
}
fn update_camera(
    mut query_cam: Query<&mut Transform, With<CameraMarker>>,
    query_player: Query<&Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
) {
    let mut cam = query_cam.single_mut();
    let player = query_player.single();
    cam.translation = player.translation;
}

fn draw_dash_distance(
    query: Query<&Movement, With<PlayerMarker>>,
    mut query_text: Query<&mut Text>,
) {
    let movement = query.single();
    let mut text = query_text.single_mut();
    text.sections[1].value = movement.dash.distance.to_string();
}
