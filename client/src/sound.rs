//! # Music System
//!
//! The music system is contextual and precedural.
//! Themes will fade in and out based on different
//! things and triggered by certain events. Each biome
//! has a core theme, and is accompanied by various
//! flourishes. For example:
//!
//!  - When the player finds a rare item, a little
//!    celebratory jingle plays.
//!  - When a player is far away from civilization,
//!    the music becomes more sparse and wondrous.
//!  - At night the music becomes more muted.
//!  - In tense situations, the music becomes higher paced.
//!
//! The core theme will have a number of tracks that
//! start and stop to keep the music interesting.

use amethyst::{
    assets::{Loader},
    audio::{WavFormat, SourceHandle},
    ecs::{World},
};
use std::{iter::Cycle, vec::IntoIter};

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, WavFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This includes the background track and the
/// sound effects.
pub fn init_audio(world: &mut World) {
    let music_files = vec!["sound/music/adventure.wav"];

    let music = {
        let loader = world.read_resource::<Loader>();
        let music = music_files
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        Music { music }
    };

    //  We have to do this in another scope because world won't let
    // us insert new resources as long as `Loader` is borrowed.
    world.add_resource(music)
}
