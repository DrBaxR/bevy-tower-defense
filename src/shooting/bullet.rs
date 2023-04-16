use std::time::Duration;

use bevy::prelude::*;

use crate::{health::DamageDealer, lifetime::Lifetime};

use super::{Shooter, Shootable};

#[derive(Reflect, Component)]
pub struct Bullet;

#[derive(Reflect, Component)]
pub struct BulletShooter;

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub shootable: Shootable,
    pub lifetime: Lifetime,
    #[bundle]
    pub sprite: SpriteBundle,
    pub damage_dealer: DamageDealer,
    pub bullet: Bullet,
}

impl BulletBundle {
    pub fn new(position: Vec3, target: &Vec3, speed: f32) -> Self {
        let trajectory =
            Vec2::new(target.x - position.x, target.y - position.y).normalize() * speed;

        BulletBundle {
            name: Name::new("Bullet"),
            shootable: Shootable { trajectory },
            lifetime: Lifetime {
                timer: Timer::new(Duration::from_millis(5000), TimerMode::Once),
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    ..default()
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(10., 10., 1.)),
                ..default()
            },
            damage_dealer: DamageDealer { damage: 10. },
            bullet: Bullet,
        }
    }
}

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
