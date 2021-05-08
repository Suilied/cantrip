use bevy::prelude::*;

mod actors;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(actors::player::setup_player.system())

        .add_system(actors::player::player_movement.system())
        .add_system(actors::player::update_manabars.system())
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}