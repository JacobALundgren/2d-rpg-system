use bevy::prelude::*;

#[derive(Default)]
pub struct Player {}

pub struct PlayerPlugin;

const WORLD_WIDTH: f32 = 1280.;
const WORLD_HEIGHT: f32 = 720.;
const PLAYER_SIDE: f32 = 60.;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0., 0., 1.).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::new(Vec2::new(PLAYER_SIDE, PLAYER_SIDE)),
            ..Default::default()
        })
        .insert(Player::default());
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(player_movement_system.system());
    }
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    if keyboard_input.get_pressed().len() == 0 {
        return;
    }
    const SPEED: f32 = 384.;

    if let Ok((_, mut transform)) = player_query.single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.;
        }

        let direction = direction.normalize_or_zero();
        transform.translation += (direction * SPEED * time.delta_seconds(), 0.).into();
        let min_point = Vec3::new(
            -(WORLD_WIDTH - PLAYER_SIDE) / 2.,
            -(WORLD_HEIGHT - PLAYER_SIDE) / 2.,
            0.,
        );
        let max_point = -min_point;
        transform.translation = transform.translation.clamp(min_point, max_point);
    }
}
