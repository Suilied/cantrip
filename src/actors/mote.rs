use bevy::prelude::*;

pub use super::character::Character;
pub use super::player::Player;

pub struct SpawnMoteEvent(pub Affinity);

#[derive(Copy, Clone)]
pub enum Affinity {
    Anger,
    Joy,
    Fear,
    Sadness,
}

impl Default for Affinity {
    fn default() -> Self {
        Affinity::Anger
    }
}

pub struct Magicmats {
    anger_mat: Handle<ColorMaterial>,
    fear_mat: Handle<ColorMaterial>,
    joy_mat: Handle<ColorMaterial>,
    sadness_mat: Handle<ColorMaterial>,
}

impl Magicmats {
    fn get_material(&self, affinity: Affinity) -> &Handle<ColorMaterial> {
        match affinity {
            Affinity::Anger => &self.anger_mat,
            Affinity::Joy => &self.joy_mat,
            Affinity::Fear => &self.fear_mat,
            Affinity::Sadness => &self.sadness_mat,
        }
    }
}

#[derive(Default)]
pub struct Mote {
    pub posx: f32,
    pub posy: f32,
    pub movevec: Vec3,
    pub affinity: Affinity,
}

pub fn setup_mote_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.insert_resource(Magicmats {
        anger_mat: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        fear_mat: materials.add(Color::rgb(0.1, 0.9, 0.1).into()),
        joy_mat: materials.add(Color::rgb(0.9, 0.9, 0.1).into()),
        sadness_mat: materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
    });
}

pub fn mote_system(
    mut commands: Commands,
    magicmats: Res<Magicmats>,
    mut er_spawn_mote: EventReader<SpawnMoteEvent>,
    mut qmotes: Query<(&mut Transform, &mut Sprite, &mut Mote)>,
    mut qplayer: Query<(&mut Character, With<Player>)>,
) {
    let (player_character, _isplayer) = qplayer.single_mut().expect("ther can be only one player!");

    for (mut trans, mut _spr, mut mote) in qmotes.iter_mut() {
        let mote_position = Vec3::new(mote.posx, mote.posy, 0.);
        let player_position = Vec3::new(player_character.posx, player_character.posy, 0.);
        mote.movevec = (mote_position - player_position).cross(Vec3::Z) * 0.02;

        mote.posx += mote.movevec.x;
        mote.posy += mote.movevec.y;

        trans.translation.x = mote.posx;
        trans.translation.y = mote.posy;
    }

    for er in er_spawn_mote.iter() {
        commands.spawn_bundle(SpriteBundle {
            material: magicmats.get_material(er.0).clone(),
            sprite: Sprite{
                size: Vec2::new(10.,10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mote {
            posx: player_character.posx,
            posy: player_character.posy,
            affinity: er.0,
            ..Default::default()
        });
    }
}