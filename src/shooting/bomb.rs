use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{
    health::{DamageDealer, Damageable},
    lifetime::Lifetime,
};

use super::{bundle::BombBundle, Shooter};

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct BombShooter;

pub fn shoot_bomb(
    mut commands: Commands,
    time: Res<Time>,
    mut shooters: Query<(&Transform, &mut Shooter, &BombShooter)>,
) {
    for (transform, mut shooter, _) in shooters.iter_mut() {
        shooter.cooldown.tick(time.delta());

        if let Some(target) = shooter.target {
            if shooter.cooldown.finished() {
                let distance = transform.translation.distance(target);

                if distance < shooter.range {
                    commands.spawn(BombBundle::new(
                        transform.translation.clone(),
                        &target,
                        300.,
                    ));
                }
            }
        }
    }
}

// spawn an explosion when colliding with a damageable
pub fn explode_bomb(
    mut commands: Commands,
    mut bombs: Query<(Entity, &Transform, &mut Bomb)>,
    mut damageables: Query<(&Transform, &mut Damageable)>,
) {
    for (bomb_entity, bomb_transform, _) in bombs.iter_mut() {
        let bomb_size = Vec2::new(bomb_transform.scale.x, bomb_transform.scale.y);

        for (damageable_transform, _) in damageables.iter_mut() {
            let damageable_size =
                Vec2::new(damageable_transform.scale.x, damageable_transform.scale.y);

            if (collide(
                bomb_transform.translation,
                bomb_size,
                damageable_transform.translation,
                damageable_size,
            ))
            .is_some()
            {
                let explosion_transform = Transform {
                    translation: damageable_transform.translation,
                    scale: Vec3::new(100., 100., 1.),
                    ..default()
                };

                commands.entity(bomb_entity).despawn();
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            ..default()
                        },
                        transform: explosion_transform.clone(),
                        ..default()
                    },
                    Lifetime {
                        timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
                    },
                ));

                commands.spawn((
                    TransformBundle {
                        local: explosion_transform.clone(),
                        ..default()
                    },
                    DamageDealer { damage: 50. },
                ));
            }
        }
    }
}
