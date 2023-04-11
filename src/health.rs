use bevy::prelude::*;

#[derive(Reflect, Component)]
pub struct Damageable {
    pub max_health: f32,
    pub health: f32,
    pub delta: f32,
}

// TODO: debug health bar

fn apply_delta(mut damageables: Query<&mut Damageable>) {
    for mut damageable in damageables.iter_mut() {
        damageable.health += damageable.delta;
        damageable.delta = 0.;
    }
}

fn print_health(damageables: Query<(&Name, &Damageable)>) {
    println!("health tick:");
    for (name, damageable) in damageables.iter() {
        println!("{} - {}/{}", name, damageable.health, damageable.max_health);
    }
}

pub struct HealthPlugin {
    pub debug: bool,
}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_delta).register_type::<Damageable>();

        if self.debug {
            app.add_system(print_health);
        }
    }
}
