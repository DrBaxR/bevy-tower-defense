use std::time::Duration;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use bullet::*;
use cursor::*;
use lifetime::*;

pub mod bullet;
pub mod cursor;
pub mod lifetime;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
    ));
}

pub fn setup_entities(mut commands: Commands) {
    commands.spawn((
        Name::new("Cursor"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(10., 10., 1.)),
            ..default()
        },
        Cursor,
    ));

    commands.spawn((
        Name::new("Enemy"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(30., 30., 1.)),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Bullet"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(10., 10., 1.)),
            ..default()
        },
        Bullet {
            trajectory: Vec2::new(50., 0.),
        },
        Lifetime {
            timer: Timer::new(Duration::from_millis(10000), TimerMode::Once),
        },
    ));
}
