use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct SkyboxTexture {
    texture_handle: Handle<Image>,
    is_loaded: bool,
}

pub fn create_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(20., 0.1, 20.))),
            material: materials.add(Color::rgb_u8(255, 255, 255).into()),
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: -0.05,
                z: 0.,
            }),
            visibility: Visibility::Visible,
            ..Default::default()
        },
        RigidBody::Fixed,
        Sleeping::disabled(),
    ));
}

pub fn create_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox = asset_server.load("blank_map_skybox.png");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.75, 0.),
            ..Default::default()
        },
        Skybox(skybox.clone()),
    ));
    commands.insert_resource(SkyboxTexture {
        texture_handle: skybox,
        is_loaded: false,
    });
}

pub fn texture_skybox(mut images: ResMut<Assets<Image>>, mut texture: ResMut<SkyboxTexture>) {
    let image = images.get_mut(&texture.texture_handle);
    if let Some(image) = image {
        if !texture.is_loaded {
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });
            texture.is_loaded = true;
        }
    }
}

pub fn create_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        visibility: Visibility::Visible,
        ..Default::default()
    });
}
