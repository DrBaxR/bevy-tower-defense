use std::time::Duration;

use bevy::prelude::*;

use crate::{lifetime::Lifetime, health::DamageDealer};

use super::Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    pub name: Name,
    pub bullet: Bullet,
    pub lifetime: Lifetime,
    #[bundle]
    pub sprite: SpriteBundle,
    pub damage_dealer: DamageDealer,
}

impl BulletBundle {
    pub fn new(position: Vec3, target: &Vec3, speed: f32) -> Self {
        let trajectory =
            Vec2::new(target.x - position.x, target.y - position.y).normalize() * speed;

        BulletBundle {
            name: Name::new("Bullet"),
            bullet: Bullet { trajectory },
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
        }
    }
}