use std::time::Duration;

use bevy::{ecs::query::QueryIter, prelude::*};

use crate::{health::DamageDealer, lifetime::Lifetime};

#[derive(Reflect, Component)]
pub struct Bullet {
    pub trajectory: Vec2,
}

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

fn update_bullet_position(time: Res<Time>, mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x =
            transform.translation.x + bullet.trajectory.x * time.delta_seconds();
        transform.translation.y =
            transform.translation.y + bullet.trajectory.y * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct Shooter {
    pub cooldown: Timer,
    pub range: f32,
    pub target: Option<Vec3>,
}

#[derive(Component)]
pub struct Targetable;

fn shoot_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut shooters: Query<(&Transform, &mut Shooter)>,
) {
    for (transform, mut shooter) in shooters.iter_mut() {
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

fn compute_target(
    mut shooters: Query<(&Transform, &mut Shooter)>,
    targetables: Query<&Transform, With<Targetable>>,
) {
    for (transform, mut shooter) in shooters.iter_mut() {
        let shooter_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let targetables_iter = targetables.iter();

        shooter.target = get_shortest_distance_for(&shooter_pos, targetables_iter);
    }
}

fn get_shortest_distance_for(
    pos: &Vec2,
    mut targetables_iter: QueryIter<&Transform, With<Targetable>>,
) -> Option<Vec3> {
    let mut shortest: Option<Vec3> = None;

    if let Some(first) = targetables_iter.next() {
        shortest = Some(first.translation);
    }

    for targetable_transform in targetables_iter {
        let targetable_pos = Vec2::new(
            targetable_transform.translation.x,
            targetable_transform.translation.y,
        );

        if let Some(current_target) = shortest {
            let current_target = Vec2::new(current_target.x, current_target.y);

            let shooter_to_current = pos.distance(current_target);
            let shooter_to_targetable = pos.distance(targetable_pos);

            if shooter_to_targetable < shooter_to_current {
                shortest = Some(targetable_transform.translation);
            }
        } else {
            shortest = Some(targetable_transform.translation);
        }
    }

    shortest
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bullet_position)
            .add_system(compute_target)
            .add_system(shoot_bullet)
            .register_type::<Bullet>();
    }
}
