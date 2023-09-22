use crate::blank_map::SkyboxTexture;
use bevy::{core_pipeline::Skybox, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq, States, Component)]
enum MovementState {
    #[default]
    Idle,
    SlowWalk,
    Walk,
    Run,
    Sprint,
    Crawl,
    Climb,
    Jump,
}

#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq, States, Component)]
enum StanceState {
    #[default]
    Standing,
    Crouched,
    Prone,
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
