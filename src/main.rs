use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::app::AppExit;

const MAX_MOVE_SPEED: f32 = 2.;

struct Player {
    bounce_offset: f32,
    bounce_speed: f32,
    bounce_power: f32,
    posx: f32,
    posy: f32,
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut position_players: Query<(&mut Transform, &mut Player)>,
    mut exit: EventWriter<AppExit>,
) {
    for (mut transform, mut player) in position_players.iter_mut() {
        let mut stop_bounce = true;
        if keyboard_input.pressed(KeyCode::Left) {
            player.posx -= MAX_MOVE_SPEED;
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.posx += MAX_MOVE_SPEED;
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player.posy += MAX_MOVE_SPEED;
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player.posy -= MAX_MOVE_SPEED;
            stop_bounce = false;
        }
        if keyboard_input.pressed(KeyCode::Escape) {
            println!("!!!ESCAPE!!!");
            exit.send(AppExit);
        }

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
        if player.bounce_offset > 0. { player.bounce_speed -= 0.08; }

        transform.translation.x = player.posx;
        transform.translation.y = player.posy + player.bounce_offset;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("sprites/player.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    })
    .insert(Player{
        bounce_offset: 0.,
        bounce_speed: 0.,
        bounce_power: 2.,
        posx: 0.,
        posy: 0.,
    });
}
