use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Reflect, Component)]
pub struct Damageable {
    pub max_health: f32,
    pub health: f32,
    pub delta: f32,
}

#[derive(Component)]
pub struct DamageDealer {
    pub damage: f32,
}

fn apply_damage_on_collision(
    mut commands: Commands,
    mut damageables: Query<(&mut Damageable, &Transform)>,
    damage_dealers: Query<(Entity, &DamageDealer, &Transform)>,
) {
    for (damage_dealer_entity, damage_dealer, damage_dealer_transform) in damage_dealers.iter() {
        let damage_dealer_size = Vec2::new(
            damage_dealer_transform.scale.x,
            damage_dealer_transform.scale.y,
        );

        for (mut damageable, damageable_transform) in damageables.iter_mut() {
            let damageable_size =
                Vec2::new(damageable_transform.scale.x, damageable_transform.scale.y);

            if collide(
                damageable_transform.translation,
                damageable_size,
                damage_dealer_transform.translation,
                damage_dealer_size,
            )
            .is_some()
            {
                damageable.delta -= damage_dealer.damage;
            }
        }

        commands.entity(damage_dealer_entity).despawn();
    }
}

fn apply_delta(mut commands: Commands, mut damageables: Query<(Entity, &mut Damageable)>) {
    for (entity, mut damageable) in damageables.iter_mut() {
        damageable.health += damageable.delta;
        damageable.delta = 0.;

        if damageable.health <= 0. {
            commands.entity(entity).despawn();
        }
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
        app.add_system(apply_delta)
            .add_system(apply_damage_on_collision)
            .register_type::<Damageable>();

        if self.debug {
            app.add_system(print_health);
        }
    }
}
