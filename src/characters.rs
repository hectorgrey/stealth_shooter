use bevy::prelude::*;

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
