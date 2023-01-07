use bevy::prelude::*;

#[derive(Reflect, Component)]
pub struct Lifetime {
    pub timer: Timer,
}

fn despawn_lifetime_entities(
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut lifetime) in lifetimes.iter_mut() {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_lifetime_entities)
            .register_type::<Lifetime>();
    }
}
