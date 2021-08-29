use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::player::Player;

pub struct AreaPlugin;

const AREA_COUNT: usize = 2;

struct GameAreas {
    areas: [Area; AREA_COUNT],
}

impl Default for GameAreas {
    fn default() -> Self {
        let passage_east = Passage {
            transform: Transform::from_xyz(1280. / 2. - 15., 0., 1.),
            sprite: Sprite::new(Vec2::new(30., 80.)),
            destination: 1,
            destination_transform: Transform::from_xyz(-1280. / 2. + 75., 0., 1.),
        };
        let passage_west = Passage {
            transform: Transform::from_xyz(-1280. / 2. + 15., 0., 1.),
            sprite: Sprite::new(Vec2::new(30., 80.)),
            destination: 0,
            destination_transform: Transform::from_xyz(1280. / 2. - 75., 0., 1.),
        };
        GameAreas {
            areas: [
                Area {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    passages: vec![passage_east],
                },
                Area {
                    color: Color::rgb_u8(0, 51, 0),
                    passages: vec![passage_west],
                },
            ],
        }
    }
}

struct AreaTransitionEvent(PassageDestination);

struct PassageMaterial(Handle<ColorMaterial>);

fn area_startup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut background: ResMut<ClearColor>,
    game_areas: Res<GameAreas>,
) {
    let passage_material = PassageMaterial(materials.add(Color::rgb(0., 1., 0.).into()));
    game_areas.areas[0].load(&mut commands, passage_material.0.clone(), &mut background);
    commands.insert_resource(passage_material);
}

impl Plugin for AreaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GameAreas::default())
            .insert_resource(ClearColor(Color::rgb(1., 0., 0.)))
            .add_event::<AreaTransitionEvent>()
            .add_startup_system(area_startup_system.system())
            .add_system(area_transition_check.system())
            .add_system(area_transition.system());
    }
}

struct Passage {
    transform: Transform,
    sprite: Sprite,
    destination: usize,
    destination_transform: Transform,
}

struct PassageDestination(usize, Transform);

struct Area {
    color: Color,
    passages: Vec<Passage>,
}

impl Area {
    fn load(
        &self,
        commands: &mut Commands,
        passage_material: Handle<ColorMaterial>,
        background: &mut ResMut<ClearColor>,
    ) {
        background.0 = self.color;
        for passage in &self.passages {
            commands
                .spawn_bundle(SpriteBundle {
                    material: passage_material.clone(),
                    transform: passage.transform,
                    sprite: passage.sprite.clone(),
                    ..Default::default()
                })
                .insert(PassageDestination(
                    passage.destination,
                    passage.destination_transform,
                ));
        }
    }
}

fn area_transition_check(
    player_query: Query<(&Player, &Transform, &Sprite)>,
    passages_query: Query<(&PassageDestination, &Transform, &Sprite)>,
    mut ev_area_transition: EventWriter<AreaTransitionEvent>,
) {
    let player = player_query.single().unwrap();
    if let Some((destination, _, _)) =
        passages_query
            .iter()
            .by_ref()
            .find(|&(_, transform, sprite)| {
                collide(
                    player.1.translation,
                    player.2.size,
                    transform.translation,
                    sprite.size,
                )
                .is_some()
            })
    {
        ev_area_transition.send(AreaTransitionEvent(PassageDestination(
            destination.0,
            destination.1,
        )));
    }
}

fn area_transition(
    mut commands: Commands,
    mut player_query: Query<(&Player, &mut Transform)>,
    mut ev_area_transition: EventReader<AreaTransitionEvent>,
    game_areas: Res<GameAreas>,
    passage_material: Res<PassageMaterial>,
    mut background: ResMut<ClearColor>,
    passages: Query<(Entity, &PassageDestination)>,
) {
    if let Some(destination) = ev_area_transition.iter().next() {
        assert!(destination.0 .0 < AREA_COUNT);
        for passage in passages.iter() {
            commands.entity(passage.0).despawn();
        }
        if let Ok((_, mut transform)) = player_query.single_mut() {
            transform.translation = destination.0 .1.translation;
        }
        game_areas.areas[destination.0 .0].load(
            &mut commands,
            passage_material.0.clone(),
            &mut background,
        );
    }
}
