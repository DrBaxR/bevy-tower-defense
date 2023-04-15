use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tower_defense::{
    bullet::BulletPlugin, cursor::CursorPlugin, grid::GridPlugin, lifetime::LifetimePlugin,
    setup_camera, setup_entities, setup_tower, constantly_spawn_enemies, SpawnTimer, health::HealthPlugin,
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
        .add_plugin(GridPlugin {
            debug: false,
            cell_size: 20.,
            map_file_path: "assets/full_size.map",
        })
        .add_plugin(HealthPlugin { debug: true })
        // other
        .insert_resource(SpawnTimer(Timer::from_seconds(10., TimerMode::Repeating)))
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(setup_entities)
                .with_system(setup_tower)
        )
        .add_system(constantly_spawn_enemies)
        .run();
}
