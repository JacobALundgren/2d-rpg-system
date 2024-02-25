use bevy::prelude::*;

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
        app.add_systems(Update, (attack, attack_duration));
    }
}

fn attack(mut commands: Commands, mut events: EventReader<AttackEvent>, facing: Query<&Facing>) {
    for (attacker, direction) in events
        .read()
        .filter_map(|x| Some(x.entity).zip(facing.get(x.entity).ok()))
    {
        commands
            .spawn(SpriteBundle {
                transform: Transform::from_translation((direction.0 * 80.).extend(0.)),
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.),
                    custom_size: Some(Vec2::new(40., 40.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Attack::default())
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
