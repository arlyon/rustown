// handles game state

use amethyst::{
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    window::ScreenDimensions,
};

use std::default;

use crate::components;
use crate::systems;
use crate::util;
use crate::sound;

pub struct PausedState {
    target: Option<systems::Target>,
}

impl default::Default for PausedState {
    fn default() -> Self {
        PausedState{
            target: None,
        }
    }
}


impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut writer = data.world.write_resource::<systems::PlayerMovementTarget>();
        self.target = Some(writer.target);
        writer.target = systems::Target::None;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut writer = data.world.write_resource::<systems::PlayerMovementTarget>();
        if let Some(target) = self.target {
            writer.target = target;
        }

        self.target = None;
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Go back to the `GameplayState`.
                return Trans::Pop;
            }
        }

        // Escape isn't pressed, so we stay in this `State`.
        Trans::None
    }
}

pub struct GameplayState;

impl SimpleState for GameplayState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        // add components
        world.register::<components::Player>();

        sound::init_audio(world);
        util::init_camera(world, &dimensions);
        util::init_ui(world);

        // Init entities
        let tex = util::load_sprites(world);
        let player = util::init_players(world, &dimensions, &tex);
        world.add_resource(systems::PlayerMovementTarget {
            target: systems::Target::Entity(player),
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event)
                || (is_key_down(&event, VirtualKeyCode::LWin)
                    && is_key_down(&event, VirtualKeyCode::Q))
            {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState::default()));
            }
        }

        Trans::None
    }
}
