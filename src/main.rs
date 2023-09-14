use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use blank_map::{create_camera, create_ground, create_light};

mod characters;
mod blank_map;

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
        .add_systems(Startup, (create_camera, create_light, create_ground))
        .add_systems(Update, hello.run_if(in_state(GameState::MainMenu)))
        .run();
}
