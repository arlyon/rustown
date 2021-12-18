use bevy::{prelude::*, render::camera::Camera};

mod animation;
mod game;
mod map;
mod sound;

pub struct RustownPlugin;
impl Plugin for RustownPlugin {
    fn build(&self, app: &mut App) {
        sound::register_systems(app);
        map::register_systems(app);
        game::register_systems(app);
        animation::register_systems(app);

        app.add_startup_system(setup.system().after("game"))
            .add_system(tick.system());
    }
}

#[derive(Component)]
pub struct PrintTimer(Timer);

fn setup(assets: Res<AssetServer>, query: Query<(Entity, &Camera)>) {
    assets.watch_for_changes().unwrap();

    debug!("Setting print timer on camera");

    for c in query.iter() {
        println!("{:?}", c);
    }
}

fn tick(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            debug!("Sprites: {}", sprites.iter().count(),);
        }
    }
}
