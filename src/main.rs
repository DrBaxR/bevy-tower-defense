use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tower_defense::{
    bullet::BulletPlugin, cursor::CursorPlugin, lifetime::LifetimePlugin,
    setup_camera, setup_entities, grid::GridPlugin,
};

fn main() {
    App::new()
        // external plugins
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
        // game plugins
        .add_plugin(CursorPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(LifetimePlugin)
        .add_plugin(GridPlugin)
        // other
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(setup_entities),
        )
        .run();
}
