use nebulousengine::NebulousEngine;
use orbit::{OrbitBody, OrbitPlugin};
use bevy::prelude::*;

mod orbit;

const PLANET_SIZE_MULT: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine, OrbitPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) { 
    // earth
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Icosphere{ radius: 3.9 * PLANET_SIZE_MULT, subdivisions: 8 }.try_into().unwrap()),
            material: materials.add(Color::rgb_u8(124, 255, 144).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        OrbitBody { mass: 5.9722 * 10_f64.powf(24.0), velocity: Vec3::ZERO }
    ));

    // moon
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Icosphere{ radius: 1.0 * PLANET_SIZE_MULT, subdivisions: 8 }.try_into().unwrap()),
            material: materials.add(Color::rgb_u8(255, 144, 144).into()),
            transform: Transform::from_xyz(0.0, 0.0, 384.4),
            ..default()
        },
        OrbitBody { mass: 7.3476 * 10_f64.powf(22.0), velocity: Vec3 { x: 1000.0, y: 0.0, z: 0.0 } }
    ));

    // geo satellite
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Icosphere{ radius: 1.0 * PLANET_SIZE_MULT, subdivisions: 8 }.try_into().unwrap()),
            material: materials.add(Color::rgb_u8(144, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.0, 35.0),
            ..default()
        },
        OrbitBody { mass: 100.0, velocity: Vec3 { x: 2850.0, y: 0.0, z: 0.0 } }
    ));
    
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { color: Color::WHITE, illuminance: 10000.0, ..Default::default() },
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 800.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
