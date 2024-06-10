use std::time::Duration;

use crate::{
    level::Level,
    physics::{Acceleration, DashState, Friction, Gravity, Grounded, Movement, Velocity},
};
use bevy::{math::Vec2, prelude::*};
pub struct PlayerPlugin;
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerMarker;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                control_player,
                draw_trail_while_dashing.after(control_player),
                update_particle_timer.after(draw_trail_while_dashing),
                afterimage_while_dashing.after(draw_trail_while_dashing),
            ),
        );
    }
}

#[derive(Component)]
pub struct TrailParticle(pub Timer);

fn draw_trail_while_dashing(
    query: Query<(&Transform, &Movement), With<PlayerMarker>>,
    mut commands: Commands,
) {
    let (transform, _) = query.single();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect::from_center_size(
                    Vec2::new(transform.translation.x, transform.translation.y),
                    Vec2::splat(0.5),
                )),
                color: Color::WHITE,
                ..default()
            },
            transform: transform.clone(),
            ..default()
        },
        TrailParticle(Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once)),
    ));
}

fn afterimage_while_dashing(
    mut commands: Commands,
    query: Query<(&Transform, &Movement), With<PlayerMarker>>,
    asset_server: Res<AssetServer>,
    _: Res<Time>,
    mut local_dist: Local<f32>,
    mut local_translation: Local<Vec3>,
) {
    let (transform, movement) = query.single();
    let mut bundle = SpriteBundle {
        texture: asset_server.load("tile_0022.png"),
        ..default()
    };

    bundle.sprite.color = bundle.sprite.color.with_a(0.5);
    bundle.transform = transform.clone();
    *local_dist += (*local_translation).distance(transform.translation);
    *local_translation = transform.translation;
    if matches!(movement.dash.status, DashState::Dashing) && *local_dist >= 100. / 4. {
        *local_dist = 0.;
        commands.spawn((
            bundle,
            TrailParticle(Timer::new(Duration::from_secs_f32(0.2), TimerMode::Once)),
        ));
    }
}

fn update_particle_timer(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailParticle, &mut Sprite)>,
    mut commands: Commands,
) {
    for (entity, mut particle, mut sprite) in query.iter_mut() {
        particle.0.tick(time.delta());
        sprite.color = Color::rgba(1., 1., 1., sprite.color.a() * 0.99);
        if particle.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<Level>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0022.png"),
            transform: Transform::from_translation(Vec3::new(
                level.player_spawn_pos.x,
                level.player_spawn_pos.y,
                0.,
            )),
            ..default()
        },
        Player,
        Velocity::default(),
        Acceleration::default(),
        Gravity::default(),
        PlayerMarker,
        Friction(0.),
        Grounded(false),
        Movement::default(),
    ));
}

fn control_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Movement, With<PlayerMarker>>,
) {
    let mut temp_vec = Vec2::ZERO;
    let mut movement = query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        movement.jump = true;
    }
    if input.just_released(KeyCode::Space) {
        movement.jump = false;
    }
    if input.pressed(KeyCode::ArrowUp) {
        temp_vec += Vec2::new(-0., 1.);
    }
    if input.pressed(KeyCode::ArrowDown) {
        temp_vec += Vec2::new(0., -1.);
    }
    if input.pressed(KeyCode::ArrowLeft) {
        temp_vec += Vec2::new(-1., 0.);
    }
    if input.pressed(KeyCode::ArrowRight) {
        temp_vec += Vec2::new(1., 0.);
    }
    if input.just_pressed(KeyCode::KeyQ) && matches!(movement.dash.status, DashState::Ready) {
        movement.dash.status = DashState::Started;
    }
    movement.directional = temp_vec;
}
