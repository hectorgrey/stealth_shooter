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
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3 { x: 0., y: -0.05, z: 0. }),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        RigidBody::Fixed,
        Sleeping::disabled(),
    ));
}

pub fn create_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //let skybox = asset_server.load("blank_map_skybox.png");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.75, 0.),
            ..Default::default()
        },
        //bevy::core_pipeline::Skybox(skybox),
    ));
}

pub fn create_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        visibility: Visibility::Visible,
        ..Default::default()
    });
}
