use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::attack::{AttackEvent, Facing};

#[derive(Component, Default)]
pub struct Player {}

pub struct PlayerPlugin;

const PLAYER_SIDE: f32 = 60.;

fn setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                color: Color::srgb(0., 0., 1.),
                custom_size: Some(Vec2::new(PLAYER_SIDE, PLAYER_SIDE)),
                ..default()
            },
            ..Default::default()
        })
        .insert(Collider::cuboid(PLAYER_SIDE / 2., PLAYER_SIDE / 2.))
        .insert(ColliderMassProperties::Density(0.))
        .insert(AdditionalMassProperties::Mass(10.))
        .insert(Player::default())
        .insert(RigidBody::Dynamic)
        .insert(Facing::default())
        .insert(Velocity::default());
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (player_movement_system, actions));
    }
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &mut Facing), With<Player>>,
) {
    const SPEED: f32 = 384.;

    if let Ok((mut velocity, mut facing)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }

        if let Some(normalized) = direction.try_normalize() {
            *facing = Facing(normalized);
        }
        velocity.linvel = direction.normalize_or_zero() * SPEED;
    }
}

fn actions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<AttackEvent>,
    player: Query<Entity, With<Player>>,
) {
    let player = match player.get_single().ok() {
        Some(x) => x,
        None => return,
    };
    for &input in keyboard_input.get_just_pressed() {
        if input == KeyCode::Space {
            event_writer.send(AttackEvent { entity: player });
        };
    }
}

#[cfg(test)]
pub mod test_utils {
    use bevy::prelude::*;

    pub struct PlayerPlugin;

    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, super::setup);
        }
    }
}
