use std::time::Duration;

use crate::level::*;
use crate::player::PlayerMarker;
use crate::util::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_physics, dash_timer));
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct Friction(pub f32);
impl Default for Friction {
    fn default() -> Self {
        Friction(FRICTION)
    }
}

#[derive(Component)]
pub struct Gravity(pub f32);
impl Default for Gravity {
    fn default() -> Self {
        Gravity(GRAVITY_DEFAULT)
    }
}

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct Movement {
    pub directional: Vec2,
    pub dash: Dash,
    pub jump: bool,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            directional: Vec2::ZERO,
            dash: Dash::default(),
            jump: false,
        }
    }
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Solid {
    pub pos_x: f32,
    pub pos_y: f32,
    pub texture_file: String,
    pub bounds: HitBox,
}

#[derive(Debug)]
pub enum DashState {
    Started,
    Dashing,
    Finished,
    Ready,
    Cancelled,
}

impl Default for DashState {
    fn default() -> Self {
        Self::Ready
    }
}

pub struct Dash {
    pub dash_count: usize,
    pub dash_timer: Timer,
    pub status: DashState,
    pub distance: f32,
    pub start_point: Vec3,
}
impl Default for Dash {
    fn default() -> Self {
        Dash {
            dash_count: DEFAULT_DASH_COUNT,
            dash_timer: Timer::new(
                Duration::from_secs_f32(DEFAULT_DASH_DURATION),
                TimerMode::Once,
            ),
            status: DashState::default(),
            distance: 0.,
            start_point: Vec3::ZERO,
        }
    }
}

pub fn dash_timer(mut query: Query<&mut Movement, With<PlayerMarker>>, time: Res<Time>) {
    let mut movement = query.single_mut();
    if matches!(movement.dash.status, DashState::Dashing) {
        movement.dash.dash_timer.tick(time.delta());
        if movement.dash.dash_timer.just_finished() {
            movement.dash.status = DashState::Finished;
            movement.dash.dash_timer.reset();
        }
    }
}

impl Solid {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        return (self.pos_x - self.bounds.half_size.x) <= x
            && (self.pos_x + self.bounds.half_size.x) >= x
            && (self.pos_y - self.bounds.half_size.y <= y)
            && (self.pos_y + self.bounds.half_size.y >= y);
    }
}

fn contains(solids: &Vec<Solid>, x: f32, y: f32) -> Option<&Solid> {
    for item in solids {
        if item.contains(x, y) {
            return Some(item);
        }
    }
    None
}

fn update_physics(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut Acceleration,
            &mut Friction,
            &mut Gravity,
            &mut Grounded,
            &mut Movement,
        ),
        With<PlayerMarker>,
    >,
    level: Res<Level>,
    time: Res<Time>,
) {
    for (
        mut transform,
        mut vel,
        mut accel,
        mut friction,
        mut gravity,
        mut grounded,
        mut movement,
    ) in query.iter_mut()
    {
        // CONFIGURATIONS
        match movement.dash.status {
            DashState::Started => {
                movement.dash.start_point = transform.translation;
                let temp_vec: Vec2;
                if movement.directional.abs() == Vec2::ZERO {
                    temp_vec = vel.0.try_normalize().unwrap_or_else(|| Vec2::new(1., 0.));
                } else {
                    temp_vec = movement.directional.normalize();
                }
                movement.dash.status = DashState::Cancelled;
                if !(movement.dash.dash_count > 0) {
                    return;
                }
                movement.dash.status = DashState::Dashing;
                movement.dash.dash_count -= 1;
                vel.0 = temp_vec * INITIAL_VEL_DASH;
                accel.0 = -temp_vec * INITIAL_ACCEL_DASH;
                gravity.0 = 0.;
                friction.0 = 0.
            }
            DashState::Finished => {
                movement.dash.distance = movement.dash.start_point.distance(transform.translation);
                movement.dash.status = DashState::Ready;
                gravity.0 = GRAVITY_DEFAULT;
                accel.0.y = 0.;
            }
            DashState::Cancelled => {
                movement.dash.status = DashState::Ready;
            }
            _ => {}
        }

        if !matches!(movement.dash.status, DashState::Dashing) {
            if grounded.0 {
                movement.dash.dash_count = DEFAULT_DASH_COUNT;

                if movement.jump {
                    vel.0.y = JMP_VEL_PLAYER;
                    movement.jump = false;
                }
            }

            if movement.directional.x.abs() != 0. {
                accel.0.x = movement.directional.x.signum() * INITIAL_ACCEL_PLAYER;
                friction.0 = 0.;
            } else {
                accel.0.x = 0.;
                friction.0 = vel.0.x.signum() * FRICTION;
            }
        }

        //calculate current movement before any other physics calcs
        let move_x: f32 = (vel.0.x * time.delta_seconds())
            + (accel.0.x - friction.0) * time.delta_seconds() * time.delta_seconds() * 0.5;
        let move_y: f32 = (vel.0.y * time.delta_seconds())
            + (-gravity.0 + accel.0.y) * time.delta_seconds() * time.delta_seconds() * 0.5;

        //update velocity and accel
        vel.0.x += (accel.0.x - friction.0) * time.delta_seconds();
        if !matches!(movement.dash.status, DashState::Dashing) {
            vel.0.x = vel.0.x.clamp(-MAX_VEL_PLAYER, MAX_VEL_PLAYER);
        }
        vel.0.y += (-gravity.0 + accel.0.y) * time.delta_seconds();

        if vel.0.x >= -MIN_VEL_PLAYER
            && vel.0.x <= MIN_VEL_PLAYER
            && movement.directional.x.abs() == 0.
            && !matches!(movement.dash.status, DashState::Dashing)
        {
            vel.0.x = 0.;
            accel.0.x = 0.;
            friction.0 = 0.;
        }

        info!(
            "accel_x : {} : vel_x : {} : friction : {} : time : {} : status : {:?}",
            accel.0.x,
            vel.0.x,
            friction.0,
            time.delta_seconds(),
            movement.directional
        );

        // info!(
        //     "accel_y : {} : vel_y : {} : gravity : {} : time : {} : status : {:?}",
        //     accel.0.y,
        //     vel.0.y,
        //     gravity.0,
        //     time.delta_seconds(),
        //     movement.dash.status
        // );

        // info!(
        //     "{} {} {}",
        //     !contains(
        //         &level.grid,
        //         transform.translation.x + move_x + movement.0.x * BLOCK_SIZE.x * 0.5,
        //         transform.translation.y,
        //     ),
        //     grounded.0,
        //     move_x
        // );

        // CHECK MOVE X --------------------------------
        if contains(
            &level.grid,
            transform.translation.x + move_x + move_x.signum() * BLOCK_SIZE.x * 0.5,
            transform.translation.y,
        )
        .is_none()
        {
            // if !contains(
            //     &level.grid,
            //     transform.translation.x + move_x + move_x.signum() * BLOCK_SIZE.x * 0.5,
            //     transform.translation.y - (BLOCK_SIZE.y * 0.5),
            // )
            // .is_none()
            // {
            //     transform.translation.y += BLOCK_SIZE.y * 0.5;
            // } else if !contains(
            //     &level.grid,
            //     transform.translation.x + move_x + move_x.signum() * BLOCK_SIZE.x * 0.5,
            //     transform.translation.y + (BLOCK_SIZE.y * 0.5),
            // )
            // .is_none()
            // {
            //     transform.translation.y -= BLOCK_SIZE.y * 0.5;
            // }
            transform.translation.x += move_x;
        } else {
            // vel.0.x = 0.;
            // accel.0.x = 0.;
            friction.0 = 0.;
        }

        // info!(
        //     "{} {} {}",
        //     !contains(
        //         &level.grid,
        //         transform.translation.x,
        //         transform.translation.y + move_y + move_y.signum() * BLOCK_SIZE.y * 0.5,
        //     ),
        //     grounded.0,
        //     move_y
        // );

        // CHECK MOVE Y --------------------------------
        if contains(
            &level.grid,
            transform.translation.x,
            transform.translation.y + move_y + move_y.signum() * BLOCK_SIZE.y * 0.5,
        )
        .is_none()
        {
            // if !contains(
            //     &level.grid,
            //     transform.translation.x - (BLOCK_SIZE.x * 0.5),
            //     transform.translation.y + move_y + move_y.signum() * BLOCK_SIZE.y * 0.5,
            // )
            // .is_none()
            // {
            //     transform.translation.x += BLOCK_SIZE.x * 0.5;
            // } else if !contains(
            //     &level.grid,
            //     transform.translation.x + (BLOCK_SIZE.x * 0.5),
            //     transform.translation.y + move_y + move_y.signum() * BLOCK_SIZE.y * 0.5,
            // )
            // .is_none()
            // {
            //     transform.translation.x -= BLOCK_SIZE.x * 0.5;
            // }
            transform.translation.y += move_y;
            grounded.0 = false;
        } else {
            // vel.0.y = 0.;
            // accel.0.y = 0.;
            grounded.0 = true;
        }
    }
}
