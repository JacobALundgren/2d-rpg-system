use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

const WORLD_WIDTH: f32 = 1280.;
const WORLD_HEIGHT: f32 = 720.;

fn no_gravity(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::ZERO;
}

fn add_area_bounds(mut commands: Commands) {
    const BOUND_THICKNESS: f32 = 1.;
    let mut bounds: Vec<(Vec2, Rot, Collider)> = Vec::with_capacity(4);
    bounds.push((
        Vec2::new(-(WORLD_WIDTH + BOUND_THICKNESS) / 2., 0.),
        0.,
        Collider::cuboid(BOUND_THICKNESS / 2., WORLD_HEIGHT / 2.),
    ));
    bounds.push((
        Vec2::new((WORLD_WIDTH + BOUND_THICKNESS) / 2., 0.),
        0.,
        Collider::cuboid(BOUND_THICKNESS / 2., WORLD_HEIGHT / 2.),
    ));
    bounds.push((
        Vec2::new(0., (WORLD_HEIGHT + BOUND_THICKNESS) / 2.),
        0.,
        Collider::cuboid(WORLD_WIDTH / 2., BOUND_THICKNESS / 2.),
    ));
    bounds.push((
        Vec2::new(0., -(WORLD_HEIGHT + BOUND_THICKNESS) / 2.),
        0.,
        Collider::cuboid(WORLD_WIDTH / 2., BOUND_THICKNESS / 2.),
    ));

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::compound(bounds));
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_startup_system(no_gravity);
        app.add_startup_system(add_area_bounds);
    }
}
