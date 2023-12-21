use bevy::prelude::*;
use rpg_system_2d::{
    area::{Area, AreaIdentifier, AreaPlugin, GameAreas, Passage},
    enemy::Enemy,
    physics::PhysicsPlugin,
    player::PlayerPlugin,
};

fn get_game_areas() -> GameAreas {
    let passage_east = Passage::new(
        Transform::from_xyz(1280. / 2. - 15., 0., 1.),
        Sprite {
            color: Color::rgb(0., 1., 0.),
            custom_size: Some(Vec2::new(30., 80.)),
            ..default()
        },
        1.into(),
        Transform::from_xyz(-1280. / 2. + 75., 0., 1.),
    );
    let passage_west = Passage::new(
        Transform::from_xyz(-1280. / 2. + 15., 0., 1.),
        Sprite {
            color: Color::rgb(0., 1., 0.),
            custom_size: Some(Vec2::new(30., 80.)),
            ..default()
        },
        0.into(),
        Transform::from_xyz(1280. / 2. - 75., 0., 1.),
    );
    GameAreas::new(vec![
        Area::new(Color::rgb(0.1, 0.1, 0.1), vec![passage_east]),
        Area::new(Color::rgb_u8(0, 51, 0), vec![passage_west]),
    ])
}

fn main() {
    App::new()
        .insert_resource(get_game_areas())
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(AreaPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_enemy)
        .run();
}

fn create_enemy(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(260., 260., 0.),
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                custom_size: Some(Vec2::new(60., 60.)),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(AreaIdentifier(1))
        .insert(Enemy::default());
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
