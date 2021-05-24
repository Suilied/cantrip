use bevy::prelude::*;

use super::player::MainCam;

const BGWIDTH: i32 = 7;
const BGHEIGHT: i32 = 5;

const TXWIDTH: f32 = 256.;
const TXHEIGHT: f32 = 256.;

pub struct Tile;

fn get_offset(window: &mut Window, center: Vec3) -> (f32, f32) {
    let x = ((-(window.width() / 2.) + center.x)/TXWIDTH).floor() * TXWIDTH;
    let y = ((-(window.height() / 2.) + center.y)/TXHEIGHT).floor() * TXHEIGHT;
    (x, y)
}

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let texture_handle = asset_server.load("sprites/background.png");

    for _i in 0..BGWIDTH*BGHEIGHT {

        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            ..Default::default()
        })
        .insert(Tile);
    }
}

pub fn bg_position_system(
    mut windows: ResMut<Windows>,
    mut cam: Query<&MainCam>,
    mut bg_tiles: Query<(&mut Transform, With<Tile>)>,
) {
    let maincam = cam.single_mut().expect("there can be only one main-camera!");
    let window = windows.get_primary_mut().unwrap();
    let mut iterator: i32 = 0;
    let (offsetx, offsety) = get_offset(window, maincam.position);

    for (mut transform, _is_tile) in bg_tiles.iter_mut() {
        transform.translation.x = offsetx + ((iterator%BGWIDTH) as f32 * TXWIDTH);
        transform.translation.y = offsety + ((iterator/BGWIDTH) as f32 * TXHEIGHT);
        //transform.translation.z = 1.;
        iterator+=1;
    }
}