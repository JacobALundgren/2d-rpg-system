use bevy::prelude::*;
use rpg_system_2d::{
    area::{Area, AreaIdentifier, AreaPlugin, GameAreas, Passage},
    enemy::Enemy,
    player::PlayerPlugin,
};

fn get_game_areas() -> GameAreas {
    let passage_east = Passage::new(
        Transform::from_xyz(1280. / 2. - 15., 0., 1.),
        Sprite::new(Vec2::new(30., 80.)),
        1.into(),
        Transform::from_xyz(-1280. / 2. + 75., 0., 1.),
    );
    let passage_west = Passage::new(
        Transform::from_xyz(-1280. / 2. + 15., 0., 1.),
        Sprite::new(Vec2::new(30., 80.)),
        0.into(),
        Transform::from_xyz(1280. / 2. - 75., 0., 1.),
    );
    GameAreas::new(vec![
        Area::new(Color::rgb(0.1, 0.1, 0.1), vec![passage_east]),
        Area::new(Color::rgb_u8(0, 51, 0), vec![passage_west]),
    ])
}

fn main() {
    App::build()
        .insert_resource(get_game_areas())
        .add_plugins(DefaultPlugins)
        .add_plugin(AreaPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_enemy.system())
        .run();
}

fn create_enemy(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_xyz(260., 260., 0.),
            sprite: Sprite::new(Vec2::new(60., 60.)),
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AreaIdentifier(1))
        .insert(Enemy::default());
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
