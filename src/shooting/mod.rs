use bevy::prelude::*;

use self::bullet::{compute_target, shoot_bullet};

pub mod bullet;
pub mod bundle;

#[derive(Reflect, Component)]
pub struct Shootable {
    pub trajectory: Vec2,
}

fn update_shootable_position(time: Res<Time>, mut bullets: Query<(&mut Transform, &Shootable)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x =
            transform.translation.x + bullet.trajectory.x * time.delta_seconds();
        transform.translation.y =
            transform.translation.y + bullet.trajectory.y * time.delta_seconds();
    }
}


#[derive(Reflect, Component)]
pub struct BulletShooter;

#[derive(Component)]
pub struct Shooter {
    pub cooldown: Timer,
    pub range: f32,
    pub target: Option<Vec3>,
}

#[derive(Component)]
pub struct Targetable;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_shootable_position)
            .add_system(compute_target)
            .add_system(shoot_bullet)
            .register_type::<Shootable>();
    }
}