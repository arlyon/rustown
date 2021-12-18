#![warn(
    clippy::todo,
    clippy::unwrap_used,
    clippy::unused_self,
    clippy::unimplemented,
    clippy::trivially_copy_pass_by_ref,
    clippy::panic
)]
#![forbid(unsafe_code)]

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use rustown::RustownPlugin;

mod rustown;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(RustownPlugin) // debug sprite counter
        .run();
}
