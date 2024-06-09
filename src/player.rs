use std::time::Duration;

use crate::physics::{Acceleration, DashState, Friction, Gravity, Grounded, Movement, Velocity};
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
            ),
        );
    }
}

#[derive(Component)]
pub struct TrailParticle(pub Timer);

fn draw_trail_while_dashing(query: Query<&Transform, With<PlayerMarker>>, mut commands: Commands) {
    let transform = query.single();
    //if matches!(movement.dash.status, DashState::Dashing) {
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
    //}
}

fn update_particle_timer(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailParticle, &mut Sprite)>,
    mut commands: Commands,
) {
    for (entity, mut particle, mut sprite) in query.iter_mut() {
        particle.0.tick(time.delta());
        let alpha =
            1.0 - (particle.0.elapsed().as_secs_f32() / particle.0.duration().as_secs_f32());
        sprite.color = Color::rgba(1., 1., 1., alpha);
        if particle.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0022.png"),
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
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
    if input.pressed(KeyCode::Space) {
        temp_vec += Vec2::new(0., 1.);
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
    if input.just_pressed(KeyCode::KeyQ) {
        movement.dash.status = DashState::Started;
    }
    movement.directional = temp_vec;
}
