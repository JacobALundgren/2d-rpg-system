use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .insert(Velocity::default());
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement_system);
    }
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Velocity)>,
) {
    const SPEED: f32 = 384.;

    if let Ok((_, mut velocity)) = player_query.get_single_mut() {
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

        direction = direction.normalize_or_zero();
        direction *= SPEED;
        velocity.linvel = direction;
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
