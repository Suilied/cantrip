use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::app::AppExit;

const MAX_MOVE_SPEED: f32 = 1.;

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct Player;
struct Palette {
    player_color: Handle<ColorMaterial>,
}
struct PlayerSprites {
    spr_player: Handle<Image>,
}

fn main() {
    let window = WindowDescriptor {
        title: "It's a Snake!".to_string(),
        width: 500.,
        height: 500.,
        ..Default::default()
    };

    App::build()
        .insert_resource(window)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
        .add_system(player_movement.system())
        // .add_system_set_to_stage(
        //     CoreStage::PostUpdate,
        //     SystemSet::new()
        //         .with_system(position_translation.system())
        //         .with_system(size_scaling.system()),
        // )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut palette: ResMut<Assets<ColorMaterial>>
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Palette {
        player_color: palette.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn spawn_player(mut commands: Commands, palette: Res<Palette>) {
    commands.spawn_bundle(SpriteBundle {
        material: palette.player_color.clone(),
        sprite: Sprite::new(Vec2::new(10., 10.)),
        ..Default::default()
    })
    .insert(Player)
    .insert(Size::square(0.8));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_positions: Query<&mut Transform, With<Player>>,
    mut exit: EventWriter<AppExit>,
) {
    for mut transform in player_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= MAX_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += MAX_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += MAX_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= MAX_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Escape) {
            println!("!!!ESCAPE!!!");
            exit.send(AppExit);
            //rage_quit();
        }
    }
}