use bevy::prelude::*;
use bevy::app::AppExit;

pub use super::character::Character;
pub use super::mote::*;

pub struct MainCam {
    pub position: Vec3
}

const MAX_MOVE_SPEED: f32 = 120.;
const STARTING_CASTSPEED: i32 = 1;

pub struct ManaBar {
    size: f32,
    pos: Vec2,
    affinity: Affinity,
}

pub struct Player;

pub fn player_update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut position_players: Query<(&mut Transform, &mut Sprite, (&mut Character, With<Player>))>,
    mut manabars: Query<&mut ManaBar>,
    mut ew_spawn_mote: EventWriter<SpawnMoteEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for (mut transform, mut sprite, (mut player, playerid)) in position_players.iter_mut() {

        let mut stop_bounce = true;
        if keyboard_input.pressed(KeyCode::Left) {
            player.posx -= MAX_MOVE_SPEED*time.delta_seconds();
            stop_bounce = false;

            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.posx += MAX_MOVE_SPEED*time.delta_seconds();
            stop_bounce = false;

            sprite.flip_x = false;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player.posy += MAX_MOVE_SPEED*time.delta_seconds();
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player.posy -= MAX_MOVE_SPEED*time.delta_seconds();
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Escape) {
            println!("!!!ESCAPE!!!");
            exit.send(AppExit);
        }

        if keyboard_input.just_pressed(KeyCode::Space){
            player.casting = !player.casting;
            let cast_string = if player.casting {"Player is casting!"} else { "player stopped casting.." };
            println!("{}", cast_string);
            println!("Motes: \nAnger: {} | Fear   : {}\nJoy  : {} | Sadness: {}", player.anger, player.fear, player.joy, player.sadness);
        }

        if keyboard_input.just_pressed(KeyCode::W) {
            // Spawn Anger Mote
            ew_spawn_mote.send(SpawnMoteEvent(Affinity::Anger));
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            // Spawn Fear Mote
            ew_spawn_mote.send(SpawnMoteEvent(Affinity::Fear));
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            // Spawn Joy Mote
            ew_spawn_mote.send(SpawnMoteEvent(Affinity::Joy));
        }

        if keyboard_input.just_pressed(KeyCode::D) {
            // Spawn Sadness Mote
            ew_spawn_mote.send(SpawnMoteEvent(Affinity::Sadness));
        }

        for mut mbars in manabars.iter_mut() {
            match mbars.affinity {
                Affinity::Anger => mbars.size = player.anger,
                Affinity::Joy => mbars.size = player.joy,
                Affinity::Fear => mbars.size = player.fear,
                Affinity::Sadness => mbars.size = player.sadness,
            }
        }

        // DO BOUNCE
        let start_bounce = keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::Down);
        if stop_bounce && player.bounce_speed > 0. {
            player.bounce_speed = 0.;
        }
        if start_bounce && player.bounce_speed == 0. {
            player.bounce_speed = player.bounce_power;
        }
        if player.bounce_speed < 0. && player.bounce_offset < 0. {
            if stop_bounce == false { 
                player.bounce_speed = player.bounce_power;
            } else {
                player.bounce_speed = 0.;
            }
            player.bounce_offset = 0.;
        }
        player.bounce_offset += player.bounce_speed;
        if player.bounce_offset > 0. { player.bounce_speed -= 4.8*time.delta_seconds(); }

        // UPDATE SPRITE
        transform.translation.x = player.posx;
        transform.translation.y = player.posy + player.bounce_offset;
    }
}


pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/player.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform: Transform {
            translation: Vec3::new(0.,0.,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Character{
        bounce_offset: 0.,
        bounce_speed: 0.,
        bounce_power: 2.,
        posx: 0.,
        posy: 0.,

        casting: false,
        castspeed: STARTING_CASTSPEED,

        anger: 0.,
        joy: 0.,
        fear: 0.,
        sadness: 0.,
    })
    .insert(Player);

    
    let barsize = Vec2::new(10., 10.);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        sprite: Sprite{
            size: barsize,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ManaBar{
        size: barsize.x,
        pos: Vec2::new(10.,10.),
        affinity: Affinity::Anger,
    })
    .insert(Affinity::Anger);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.9, 0.9, 0.1).into()),
        sprite: Sprite{
            size: barsize,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ManaBar{
        size: barsize.x,
        pos: Vec2::new(25.,10.),
        affinity: Affinity::Joy,
    })
    .insert(Affinity::Joy);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.1, 0.9, 0.1).into()),
        sprite: Sprite{
            size: barsize,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ManaBar{
        size: barsize.x,
        pos: Vec2::new(40.,10.),
        affinity: Affinity::Fear,
    })
    .insert(Affinity::Fear);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
        transform: Transform {
            translation: Vec3::new(55.,10.,0.),
            ..Default::default()
        },
        sprite: Sprite{
            size: barsize,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ManaBar{
        size: barsize.x,
        pos: Vec2::new(55.,10.),
        affinity: Affinity::Sadness,
    })
    .insert(Affinity::Sadness);
}

pub fn update_manabars(
    mut manabars: Query<(&mut Transform, &mut Sprite, &mut ManaBar)>,
) {
    // change this so that it's drawn on the UI layer
    for (mut transform, mut manasprite, bar) in manabars.iter_mut() {
        manasprite.size.y = bar.size;
        transform.translation.x = bar.pos.x; 
        transform.translation.y = bar.pos.y;
    }
}

pub fn update_camera(
    mut player: Query<(&mut Character, With<Player>)>,
    mut cam: Query<(&mut Transform, &mut MainCam)>,
) {
    let (character, _isplayer) = player.single_mut().expect("there can be only one player!");
    let (mut camtransform, mut maincam) = cam.single_mut().expect("there can be only one main-camera!");

    // Update actual camera
    camtransform.translation.x = character.posx;
    camtransform.translation.y = character.posy;

    // update MainCam position
    maincam.position = camtransform.translation;
}