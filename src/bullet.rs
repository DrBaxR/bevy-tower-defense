use std::time::Duration;

use bevy::prelude::*;

use crate::lifetime::Lifetime;

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
                    color: Color::RED,
                    ..default()
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(10., 10., 1.)),
                ..default()
            },
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
    pub target: Vec3,
}

fn shoot_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut shooters: Query<(&Transform, &mut Shooter)>,
) {
    for (transform, mut shooter) in shooters.iter_mut() {
        shooter.cooldown.tick(time.delta());

        if shooter.cooldown.finished() {
            commands.spawn(BulletBundle::new(
                transform.translation.clone(),
                &shooter.target,
                100.,
            ));
        }
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bullet_position)
            .add_system(shoot_bullet)
            .register_type::<Bullet>();
    }
}
