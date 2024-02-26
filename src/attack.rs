use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;

pub struct AttackPlugin;

#[derive(Event)]
pub struct AttackEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Facing(pub Vec2);

#[derive(Component)]
pub struct Attack {
    duration: Timer,
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            duration: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

impl Default for Facing {
    fn default() -> Self {
        Self(Vec2::new(1.0, 0.0))
    }
}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>();
        app.add_systems(Update, attack_duration);
        app.add_systems(Update, (attack, hit).chain());
    }
}

fn attack(mut commands: Commands, mut events: EventReader<AttackEvent>, facing: Query<&Facing>) {
    for (attacker, direction) in events
        .read()
        .filter_map(|x| Some(x.entity).zip(facing.get(x.entity).ok()))
    {
        const ATTACK_SIDE: f32 = 40.;
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_translation((direction.0 * 80.).extend(0.)),
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.),
                    custom_size: Some(Vec2::new(ATTACK_SIDE, ATTACK_SIDE)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::cuboid(ATTACK_SIDE / 2.0, ATTACK_SIDE / 2.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Attack::default())
            .insert(Sensor)
            .set_parent(attacker);
    }
}

fn attack_duration(
    mut commands: Commands,
    mut attacks: Query<(Entity, &mut Attack)>,
    time: Res<Time>,
) {
    for (entity, mut attack) in attacks.iter_mut() {
        attack.duration.tick(time.delta());
        if attack.duration.finished() {
            commands
                .get_entity(entity)
                .expect("should find entity of attack currently expiring")
                .despawn();
        }
    }
}

fn find_matching_collisions<'world, 'state, 'a, DL, DR, FL, FR>(
    l_query: Query<'world, 'state, DL, FL>,
    r_query: Query<'world, 'state, DR, FR>,
    collisions: impl Iterator<Item = &'a CollisionEvent> + 'a,
) -> impl Iterator<Item = (Entity, Entity)> + 'a
where
    DL: bevy::ecs::query::QueryData,
    DR: bevy::ecs::query::QueryData,
    FL: bevy::ecs::query::QueryFilter,
    FR: bevy::ecs::query::QueryFilter,
    'world: 'a,
    'state: 'a,
{
    let started_collisions = collisions.filter_map(|x| {
        if let CollisionEvent::Started(l, r, _) = x {
            Some((l, r))
        } else {
            None
        }
    });
    started_collisions.filter_map(move |(&x, &y)| {
        match (
            l_query.contains(x),
            l_query.contains(y),
            r_query.contains(x),
            r_query.contains(y),
        ) {
            (true, _, _, true) => Some((x, y)),
            (_, true, true, _) => Some((y, x)),
            _ => None,
        }
    })
}

fn hit(
    mut commands: Commands,
    attacks: Query<&Attack>,
    enemies: Query<Entity, With<Enemy>>,
    mut collisions: EventReader<CollisionEvent>,
) {
    let relevant_collisions = find_matching_collisions(attacks, enemies, collisions.read());
    for (_, attacked) in relevant_collisions {
        if let Some(x) = commands.get_entity(attacked) {
            x.despawn_recursive();
        }
    }
}
