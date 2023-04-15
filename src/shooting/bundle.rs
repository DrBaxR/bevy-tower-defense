use std::time::Duration;

use bevy::prelude::*;

use crate::{health::DamageDealer, lifetime::Lifetime};

use super::bomb::Bomb;
use super::bullet::Bullet;
use super::Shootable;

// TODO: move to their modules
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

#[derive(Bundle)]
pub struct BombBundle {
    pub name: Name,
    pub shootable: Shootable,
    pub lifetime: Lifetime,
    #[bundle]
    pub sprite: SpriteBundle,
    pub bomb: Bomb,
}

impl BombBundle {
    pub fn new(position: Vec3, target: &Vec3, speed: f32) -> Self {
        let trajectory =
            Vec2::new(target.x - position.x, target.y - position.y).normalize() * speed;

        BombBundle {
            name: Name::new("Bullet"),
            shootable: Shootable { trajectory },
            lifetime: Lifetime {
                timer: Timer::new(Duration::from_millis(5000), TimerMode::Once),
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(10., 10., 1.)),
                ..default()
            },
            bomb: Bomb,
        }
    }
}