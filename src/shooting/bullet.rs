use bevy::{ecs::query::QueryIter, prelude::*};

use super::{Shooter, Targetable, Bullet, bundle::BulletBundle};


pub fn update_bullet_position(time: Res<Time>, mut bullets: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x =
            transform.translation.x + bullet.trajectory.x * time.delta_seconds();
        transform.translation.y =
            transform.translation.y + bullet.trajectory.y * time.delta_seconds();
    }
}

pub fn shoot_bullet(
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

pub fn compute_target( mut shooters: Query<(&Transform, &mut Shooter)>,
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
