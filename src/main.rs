use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts,
    EguiPlugin,
};

mod characters;

fn hello(mut contexts: EguiContexts) {
    egui::Window::new("Hello World").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, hello)
        .run();
}
