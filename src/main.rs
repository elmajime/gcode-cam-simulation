use std::f32::consts;

use bevy::prelude::*;
use bevy_heightmap::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh: Handle<Mesh> = meshes.add(HeightMap {
        size: UVec2::new(100, 100),
        h: |p: Vec2| ((20. * p.x).sin() + (20. * p.y).sin()) / 2.,
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(40, 100, 10),
            ..default()
        })),
        Transform {
            scale: Vec3::new(10., 10.0, 1.),
            rotation: Quat::from_axis_angle(Vec3::X, -consts::FRAC_PI_2),
            ..default()
        },
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform {
            translation: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        },
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
