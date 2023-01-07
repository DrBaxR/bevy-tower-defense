use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component)]
struct Cursor;

#[derive(Reflect, Component)]
struct Bullet {
    trajectory: Vec2,
}

fn setup_camera(mut commands: Commands) {
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

fn setup_entities(mut commands: Commands) {
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
    ));
}

fn move_cursor_to_mouse(
    windows: Res<Windows>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut sprites: Query<&mut Transform, (With<Sprite>, With<Cursor>)>,
) {
    for event in cursor_moved.iter() {
        let window = windows.get_primary().expect("Could not get primary window");
        let position = event.position;

        for mut transform in sprites.iter_mut() {
            transform.translation = Vec3::new(
                position.x - window.width() / 2.,
                position.y - window.height() / 2.,
                0.,
            );
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                resizable: false,
                width: 1280.,
                height: 720.,
                title: String::from("Tower Defense Game"),
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(setup_entities),
        )
        .add_system(move_cursor_to_mouse)
        .add_system(update_bullet_position)
        // types for debugging in ui
        .register_type::<Bullet>()
        .run();
}
