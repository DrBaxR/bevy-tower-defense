use bevy::prelude::*;

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

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_cursor_to_mouse);
    }
}
