use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rpg_system_2d::{
    area::{Area, AreaIdentifier, AreaPlugin, GameAreas, Passage, PassageDestination},
    attack::AttackPlugin,
    enemy::Enemy,
    physics::PhysicsPlugin,
    player::PlayerPlugin,
};

fn get_game_areas() -> GameAreas {
    let passage_east = Passage::new(
        Transform::from_xyz(1280. / 2. - 15., 0., 1.),
        Sprite {
            color: Color::srgb(0., 1., 0.),
            custom_size: Some(Vec2::new(30., 80.)),
            ..default()
        },
        PassageDestination(1.into(), Transform::from_xyz(-1280. / 2. + 75., 0., 1.)),
    );
    let passage_west = Passage::new(
        Transform::from_xyz(-1280. / 2. + 15., 0., 1.),
        Sprite {
            color: Color::srgb(0., 1., 0.),
            custom_size: Some(Vec2::new(30., 80.)),
            ..default()
        },
        PassageDestination(0.into(), Transform::from_xyz(1280. / 2. - 75., 0., 1.)),
    );
    GameAreas::new(vec![
        Area::new(Color::srgb(0.1, 0.1, 0.1), vec![passage_east]),
        Area::new(Color::srgb_u8(0, 51, 0), vec![passage_west]),
    ])
}

fn main() {
    App::new()
        .insert_resource(get_game_areas())
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(AreaPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AttackPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_enemy)
        .run();
}

fn create_enemy(mut commands: Commands) {
    const ENEMY_SIDE: f32 = 60.;
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(260., 260., 0.),
            sprite: Sprite {
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(ENEMY_SIDE, ENEMY_SIDE)),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(Collider::cuboid(ENEMY_SIDE / 2.0, ENEMY_SIDE / 2.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(AreaIdentifier(1))
        .insert(Enemy);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
