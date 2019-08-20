//! Contains the logic the for entity movement and health systems

use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    core::timing::Time,
    ecs::{Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage, Entities},
    input::{InputHandler, StringBindings},
    ui::UiText,
    utils::fps_counter::FpsCounter,

};
use std::default;

use crate::components;
use crate::components::Position;

#[derive(Copy,Clone,Debug)]
pub enum Target {
    Position(Position),
    Entity(Entity),
    None,
}

/// A target to move to.
#[derive(Debug)]
pub struct PlayerMovementTarget {
    pub target: Target,
}

impl default::Default for PlayerMovementTarget {
    fn default() -> Self {
        PlayerMovementTarget {
            target: Target::None,
        }
    }
}

/// Handles the movement of the players in the game.
pub struct PlayerMovementSystem;
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, components::Player>,
        Read<'a, PlayerMovementTarget>,
        Read<'a, Time>,
    );

    // all the non-player entities move toward the player.
    fn run(&mut self, (mut positions, players, target, time): Self::SystemData) {
        let target_transform = match &target.target {
            Target::Position(p) => Some(p.pos),
            Target::Entity(e) => positions.get(*e).map(|p| p.pos),
            Target::None => None,
        };

        let target_transform = if let Some(transform) = target_transform {
            transform.to_owned()
        } else {
            return;
        };

        for (player, player_position) in (&players, &mut positions).join() {
            let direction: Vector3<f32> = target_transform - player_position.pos;
            if direction.magnitude() < 0.1 { continue };
            player_position.pos += direction.normalize() * player.speed * time.delta_real_seconds();
        }
    }
}

/// Handles the health of the players in the game.
pub struct PlayerHealthSystem;
impl<'a> System<'a> for PlayerHealthSystem {
    type SystemData = (WriteStorage<'a, components::Player>);

    fn run(&mut self, mut players: Self::SystemData) {
        for player in (&mut players).join() {
            player.health += 1;
        }
    }
}

/// Handles the controls for the players in the game.
pub struct PlayerControlSystem;
impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        ReadStorage<'a, components::Player>,
        ReadStorage<'a, components::markers::Controllable>,
        WriteStorage<'a, components::Position>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (players, controllables, mut positions, input, time): Self::SystemData) {
        for (player, _, position) in (&players, &controllables, &mut positions).join() {
            let direction = Vector3::new(
                -input.axis_value("horizontal").unwrap(),
                input.axis_value("vertical").unwrap(),
                0.0,
            );

            if direction.magnitude() < 0.01 { continue; };

            position.pos += direction.normalize() * player.speed * time.delta_real_seconds();
        }
    }
}

/// Handles the UI.
pub struct UiSystem;
impl<'a> System<'a> for UiSystem {
    type SystemData = (
        ReadExpect<'a, components::Interface>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
    );

    fn run(&mut self, (interface, mut ui_text, fps_counter): Self::SystemData) {
        if let Some(text) = ui_text.get_mut(interface.fps) {
            text.text = format!("fps: {}", fps_counter.sampled_fps());
        }
    }
}

/// A target to move to.
pub struct CameraTarget {
    pub target: Target,
}

impl default::Default for CameraTarget {
    fn default() -> Self {
        CameraTarget {
            target: Target::None,
        }
    }
}

/// Handles rendering the world.
/// todo(arlyon) optimize? hidden: bool,
pub struct WorldRenderSystem {
    pub zoom: f32,
}

impl<'a> System<'a> for WorldRenderSystem {
    type SystemData = (
        Read<'a, PlayerMovementTarget>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Position>,
        Entities<'a>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (target, mut transforms, positions, entities, input, time): Self::SystemData) {
        let camera_position = match target.target {
            Target::Entity(e) => positions.get(e).map(|f| f.pos),
            Target::Position(p) => Some(p.pos),
            Target::None => {
                for (_, ent) in (&positions, &entities).join() {
                    transforms.remove(ent);
                    println!("REMOVING TRANSFORMS")
                }
                None
            }
        };

        if camera_position.is_none() { return };
        let camera_position = camera_position.unwrap();

        self.zoom += input.axis_value("zoom").unwrap() * time.delta_real_seconds();
        let scale = Vector3::new(self.zoom, self.zoom, 1.0);

        for (_, entity_position, transform) in (&entities, &positions, &mut transforms).join() {
            let delta = (entity_position.pos - camera_position) * self.zoom * 32.0;
            transform.set_translation(delta);
            transform.set_scale(scale)
        }
    }
}