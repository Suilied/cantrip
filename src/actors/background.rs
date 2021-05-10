use bevy::prelude::*;

const BGWIDTH: i32 = 7;
const BGHEIGHT: i32 = 5;
const TXWIDTH: i32 = 256;
const TXHEIGHT: i32 = 256;

pub struct Tile{
    index: i32,
    pos: Vec3,
}

struct Boundary {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct Background {
    tiles: Vec<Vec3>,
    offset: Vec3,
    innerbound: Boundary,
    outerbound: Boundary,
}

fn getx(index: i32)-> f32 {
    ((index%BGWIDTH)*TXWIDTH) as f32
}

fn gety(index: i32) -> f32 {
    ((index/BGWIDTH)*TXHEIGHT) as f32
}

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){

    let texture_handle = asset_server.load("sprites/background.png");
    let mut newbackground = Background {
        tiles: Vec::new(),
        offset: Vec3::new(0.,0.,0.),
        innerbound: Boundary {
            left: -(256.*3.5),
            right: (256.*3.5),
            top: (256.*2.5),
            bottom: (256.*2.5),
        },
        outerbound: Boundary {
            left: -(256.*4.5),
            right: (256.*4.5),
            top: (256.*3.5),
            bottom: (256.*3.5),
        }
    };

    for tile in 0..(BGWIDTH*BGHEIGHT) {
        newbackground.tiles.push(Vec3::new(getx(tile),gety(tile),0.));

        commands.spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.clone().into()),
            transform: Transform {
                translation: Vec3::new(newbackground.tiles[tile as usize].x,newbackground.tiles[tile as usize].y,0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Tile {
            index: tile,
            pos: Vec3::new(newbackground.tiles[tile as usize].x,newbackground.tiles[tile as usize].y,0.)
         });
    }
    commands.spawn().insert(newbackground);
}

pub fn bg_position_system(
    mut background: Query<&mut Background>,
    mut bg_tiles: Query<(&mut Sprite, With<Tile>)>,
) {
    for (mut bg) in background.iter_mut() {

        for (sprite, mut istile) in bg_tiles.iter_mut() {
    
            bg.offset.x -= 0.2;
            bg.offset.y -= 0.2;
    
            //sprite.transform.translation.x = bg.offset.x + 
    
        }
    }
}

// pub fn tile_positioning(
//     background: Query<&mut Background>,
//     tilesprites: Query<(&mut Sprite, &mut Tile)>
// ) {

//     // for ( bg ) in background.single(){
//     // }

//     if let Some(bgref) = background.get_entity(){

//     }
    
//     for (sprite, tile) in tilesprites.iter_mut() {
//         sprite.transform.translation.x = 
//     }
// }