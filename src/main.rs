use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use blank_map::{create_ground, create_light, texture_skybox};
use characters::setup_player;

mod blank_map;
mod characters;

#[derive(States, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum GameState {
    #[default]
    MainMenu,
    PauseMenu,
    InGame,
}

fn hello(mut contexts: EguiContexts) {
    egui::Window::new("Hello World").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_state::<GameState>()
        .add_systems(Startup, (create_light, create_ground))
        .add_systems(Startup, setup_player)
        .add_systems(Update, texture_skybox)
        .add_systems(Update, hello.run_if(in_state(GameState::MainMenu)))
        .run();
}
