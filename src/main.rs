use bevy::prelude::*;

mod actors;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(actors::background::setup_background.system())
        .add_startup_system(actors::player::setup_player.system())
        

        .add_system(actors::player::player_movement.system())
        .add_system(actors::player::update_manabars.system())
        .add_system(actors::player::update_camera.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/background.png");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(actors::player::MainCam);
}