use std::time::Duration;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use bullet::*;
use cursor::*;
use grid::Grid;

pub mod bullet;
pub mod cursor;
pub mod lifetime;
pub mod grid;

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

    // TODO: uncomment when done with grid testing
    // commands.spawn((
    //     Name::new("Tower"),
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::WHITE,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(30., 30., 1.)),
    //         ..default()
    //     },
    //     Shooter {
    //         cooldown: Timer::new(Duration::from_millis(50000), TimerMode::Repeating),
    //         target: Vec3::ZERO,
    //     },
    // ));
}
