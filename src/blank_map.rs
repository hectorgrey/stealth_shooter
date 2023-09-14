use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn create_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(20., 0.1, 20.))),
            material: materials.add(Color::BEIGE.into()),
            ..Default::default()
        },
        RigidBody::Fixed,
    ));
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
