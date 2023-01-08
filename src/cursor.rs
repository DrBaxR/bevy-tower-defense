use bevy::prelude::*;

use crate::bullet::Shooter;

#[derive(Component)]
pub struct Cursor;

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

fn move_tower_target_to_cursor(
    cursors: Query<&Transform, With<Cursor>>,
    mut shooters: Query<&mut Shooter>,
) {
    let cursor_transform = cursors.single();

    for mut shooter in shooters.iter_mut() {
        shooter.target = cursor_transform.translation.clone();
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_cursor_to_mouse)
            .add_system(move_tower_target_to_cursor);
    }
}
