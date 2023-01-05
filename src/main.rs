use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
struct Cursor;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });

    // other
    commands.spawn((
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
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(100., 100., 1.)),
            ..default()
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
        .add_startup_system(setup)
        .add_system(move_cursor_to_mouse)
        .run();
}
