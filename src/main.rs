use bevy::prelude::*;

fn hello() {
    println!("Hello World!");
}

fn main() {
    App::new()
        .add_systems(Update, hello)
        .run();
}
