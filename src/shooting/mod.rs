use bevy::prelude::*;

use self::bullet::{update_bullet_position, compute_target, shoot_bullet};

pub mod bullet;
pub mod bundle;

#[derive(Reflect, Component)]
pub struct Bullet {
    pub trajectory: Vec2,
}


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
        app.add_system(update_bullet_position)
            .add_system(compute_target)
            .add_system(shoot_bullet)
            .register_type::<Bullet>();
    }
}