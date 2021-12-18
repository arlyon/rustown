use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use rand::{prelude::SliceRandom, Rng};

use super::{game::Velocity, sound::Sound};

#[derive(Component)]
pub enum AnimationState {
    Standard,
    Combat,
}

#[derive(Component)]
pub struct AnimationSet {
    pub idle: Handle<TextureAtlas>,
    pub up: Handle<TextureAtlas>,
    pub down: Handle<TextureAtlas>,
    pub left: Handle<TextureAtlas>,
    pub right: Handle<TextureAtlas>,
    pub attack: Handle<TextureAtlas>,
}

pub fn register_systems(app: &mut App) {
    app.add_system(tick_sprites.system())
        .add_system(manage_animation_set.system());
}

fn tick_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}

fn manage_animation_set(
    mut query: Query<(
        &AnimationSet,
        &Velocity,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        Option<&AnimationState>,
        Option<&Sound>,
    )>,
    audio: Res<Audio>,
) {
    let mut rng = rand::thread_rng();

    for (set, velocity, mut sprite, mut tas, state, sounds) in query.iter_mut() {
        let selection = match (velocity.0.x, velocity.0.y) {
            (x, y) if x.abs() < 0.01 && y.abs() < 0.01 => &set.idle,
            (x, _) if x < 0.0 => {
                tas.flip_x = true;
                &set.right
            }
            (x, _) if x > 0.0 => {
                tas.flip_x = false;
                &set.left
            }
            (_, y) if y > 0.0 => &set.up,
            (_, y) if y < 0.0 => &set.down,
            _ => &set.right,
        };

        let selection = match (state, tas.index, &sprite) {
            (_, x, s) if x != 0 && s.eq(&set.attack) => &set.attack,
            (Some(AnimationState::Combat), _, _) => &set.attack,
            _ => selection,
        };

        if *sprite != *selection {
            *sprite = selection.clone();
            tas.index = 0;
        }

        if let Some(sounds) = sounds {
            if tas.index == 0 {
                if sprite.eq(&set.right) && rng.gen_ratio(1, 3) {
                    audio.play_in_channel(
                        sounds.walk.choose(&mut rng).unwrap().clone(),
                        &AudioChannel::new("combat".to_owned()),
                    );
                }
                if sprite.eq(&set.attack) {
                    audio.play_in_channel(
                        sounds.attack.choose(&mut rng).unwrap().clone(),
                        &AudioChannel::new("combat".to_owned()),
                    );
                }
            }
        }
    }
}
