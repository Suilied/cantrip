use bevy::prelude::*;

use super::player::MainCam;

const BGWIDTH: i32 = 7;
const BGHEIGHT: i32 = 5;

const TXWIDTH: f32 = 256.;
const TXHEIGHT: f32 = 256.;

pub struct Tile{
    posx: f32,
    posy: f32,
}

fn get_offset(window: &mut Window, center: Vec3) -> (f32, f32) {
    let x = ((-(window.width() / 2.) + center.x)/TXWIDTH).floor() * TXWIDTH;
    let y = ((-(window.height() / 2.) + center.y)/TXHEIGHT).floor() * TXHEIGHT;
    (x, y)
}

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
){
    let mut window = windows.get_primary_mut().unwrap();
    let texture_handle = asset_server.load("sprites/background.png");
    let (offsetx, offsety) = get_offset(window, Vec3::new(0.,0.,0.));

    for i in 0..BGWIDTH*BGHEIGHT {
        let placex = offsetx + ((i%BGWIDTH) as f32 * TXWIDTH);
        let placey = offsety + ((i/BGWIDTH) as f32 * TXHEIGHT);

        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            ..Default::default()
        })
        .insert(Tile { posx: placex, posy: placey });

        println!("tile position: {}x, {}y", placex, placey);
    }
}

pub fn bg_position_system(
    mut windows: ResMut<Windows>,
    mut cam: Query<(&MainCam)>,
    mut bg_tiles: Query<(&mut Transform, &mut Tile)>,
) {
    let (maincam) = cam.single_mut().expect("there can be only one main-camera!");
    let mut window = windows.get_primary_mut().unwrap();
    let mut iterator: i32 = 0;
    let (offsetx, offsety) = get_offset(window, maincam.position);

    for (mut transform, mut tile) in bg_tiles.iter_mut() {
        transform.translation.x = offsetx + ((iterator%BGWIDTH) as f32 * TXWIDTH);
        transform.translation.y = offsety + ((iterator/BGWIDTH) as f32 * TXHEIGHT);
        iterator+=1;
    }
}