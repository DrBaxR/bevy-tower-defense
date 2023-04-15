use bevy::prelude::*;

use super::{Shooter, bundle::BulletBundle};

#[derive(Reflect, Component)]
pub struct Bullet;

#[derive(Reflect, Component)]
pub struct BulletShooter;

pub fn shoot_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut shooters: Query<(&Transform, &mut Shooter, &BulletShooter)>,
) {
    for (transform, mut shooter, _) in shooters.iter_mut() {
        shooter.cooldown.tick(time.delta());

        if let Some(target) = shooter.target {
            if shooter.cooldown.finished() {
                let distance = transform.translation.distance(target);

                if distance < shooter.range {
                    commands.spawn(BulletBundle::new(
                        transform.translation.clone(),
                        &target,
                        300.,
                    ));
                }
            }
        }
    }
}
