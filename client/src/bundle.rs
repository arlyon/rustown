use amethyst::audio::DjSystem;
use amethyst::core::SystemBundle;
use amethyst::ecs::DispatcherBuilder;
use amethyst::prelude::World;
use amethyst::Error;
use shrev::EventChannel;

use crate::events::WorldEvent;
use crate::{sound, systems};

#[derive(Debug)]
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.insert(EventChannel::<WorldEvent>::new());

        builder.add(systems::UiSystem, "ui_system", &[]);
        builder.add(
            DjSystem::new(|music: &mut sound::Music| music.music.next()),
            "dj_system",
            &[],
        );
        builder.add(systems::ActorMovementSystem, "movement", &[]);
        builder.add(systems::ActorHealthSystem, "health", &[]);
        builder.add(systems::PlayerControlSystem, "player_control", &[]);
        builder.add(
            systems::CameraControlSystem::new(world, 4.0),
            "camera_control",
            &["player_control"],
        );
        builder.add(
            systems::ActorRenderSystem,
            "actor_render",
            &["camera_control"],
        );
        builder.add(
            systems::WorldGenerationSystem::new(world, "deadbeef".to_string()),
            "world_generation",
            &["camera_control"],
        );
        builder.add(
            systems::WorldRenderSystem::new(world),
            "world_render",
            &["camera_control", "world_generation"],
        );

        Ok(())
    }
}
