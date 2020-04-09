use amethyst::{
    ecs::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::ActiveCamera,
    window::ScreenDimensions,
};
use shrev::EventChannel;

use crate::components::{markers::Controllable, Living, Position, Terrain};
use crate::events::WorldEvent;
use crate::sound;
use crate::states::pause::PauseState;
use crate::systems;
use crate::systems::util::Target;
use crate::util;
use crate::components::markers::Actor;

#[derive(Default)]
pub struct PlayState {
    player: Option<Entity>,
}

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // add components
        world.register::<Living>();
        world.register::<Position>();
        world.register::<Terrain>();
        world.register::<Controllable>();
        world.register::<Actor>();

        // init world
        sound::init_audio(world);
        util::init_ui(world);
        let tex = util::load_sprites(world);
        let player = util::init_players(world, &tex);
        self.player = Some(player);

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        let camera = util::init_camera(world, width, height);

        // world.insert(systems::actor_movement::ActorAITarget {
        //     target: systems::util::Target::Entity(player),
        // });

        world.insert(ActiveCamera {
            entity: Some(camera),
        });

        world.insert(systems::util::CameraTarget {
            camera: Some(camera),
            target: systems::util::Target::Entity(player, 10.0),
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            let world = data.world;

            // Check if the window should be closed
            if is_close_requested(&event)
                || (is_key_down(&event, VirtualKeyCode::LWin)
                    && is_key_down(&event, VirtualKeyCode::Q))
            {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }

            if is_key_down(&event, VirtualKeyCode::Space) {
                if let Some(player) = self.player {
                    let fetched = world.try_fetch_mut::<systems::util::CameraTarget>();
                    if let Some(mut target) = fetched {
                        log::info!("Toggle camera!");
                        target.target = match target.target {
                            Target::Entity(_, _) => Target::None,
                            Target::None => Target::Entity(player, 10.0),
                            x => x,
                        };
                    }
                }
            }

            if is_key_down(&event, VirtualKeyCode::G) {
                let c = world.get_mut::<EventChannel<WorldEvent>>();
                if let Some(dispatch) = c {
                    dispatch.single_write(WorldEvent::GenerateRequest);
                };
            }
        }

        Trans::None
    }
}
