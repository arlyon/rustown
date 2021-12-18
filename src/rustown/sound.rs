use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};
use rand::prelude::SliceRandom;

struct MusicTimer(Timer);
struct Music {
    pub bg: Vec<Handle<AudioSource>>,
}

#[derive(Clone, Component)]
pub struct Sound {
    pub cash: Vec<Handle<AudioSource>>,
    pub walk: Vec<Handle<AudioSource>>,
    pub attack: Vec<Handle<AudioSource>>,
}

pub fn register_systems(app: &mut App) {
    app.add_startup_system(setup_sound.system().label("sound"))
        .add_system(background_music.system());
}

fn setup_sound(mut commands: Commands, assets: Res<AssetServer>, audio: Res<Audio>) {
    debug!("Setting up sound");
    audio.set_volume_in_channel(0.5, &AudioChannel::new("combat".to_owned()));

    let m = Music {
        bg: [
            "sound/music/forest-1.mp3",
            "sound/music/forest-2.mp3",
            "sound/music/forest-3.mp3",
        ]
        .into_iter()
        .map(|path| assets.load(path))
        .collect(),
    };

    commands.insert_resource(m);

    let mut timer = MusicTimer(Timer::new(Duration::from_secs(60), true));
    timer.0.set_elapsed(Duration::from_secs(60));

    commands.insert_resource(timer);

    debug!("Sound complete")
}

fn background_music(
    time: Res<Time>,
    music: Res<Music>,
    audio: Res<Audio>,
    mut timer: ResMut<MusicTimer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        info!("playing music");
        let mut rng = rand::thread_rng();
        let music = music.bg.choose(&mut rng).unwrap();
        audio.play_in_channel(music.clone(), &AudioChannel::new("bg".to_owned()));

        timer.0.reset();
    }
}
