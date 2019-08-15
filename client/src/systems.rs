// contains the logic the for entity movement and health systems

use amethyst::{
    core::math::{Unit, Vector3},
    core::transform::Transform,
    ecs::{Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiText,
    utils::fps_counter::FpsCounter,
};
use std::default;

use crate::components;

#[derive(Copy,Clone)]
pub enum Target {
    // Location(Transform),
    Entity(Entity),
    None,
}

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

pub struct PlayerMovementSystem;
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, components::Player>,
        Read<'a, PlayerMovementTarget>,
    );

    // all the non-player entities move toward the player
    fn run(&mut self, (mut transforms, players, target): Self::SystemData) {
        let target_transform = match &target.target {
            // Target::Location(t) => Some(t.translation()),
            Target::Entity(e) => transforms.get(*e).map(|t| t.translation()),
            Target::None => None,
        };

        let target_transform = if let Some(transform) = target_transform {
            transform.to_owned()
        } else {
            return;
        };

        for (player, player_transform) in (&players, &mut transforms).join() {
            let direction: Vector3<f32> = target_transform - player_transform.translation();
            let direction = if direction.magnitude() > 5.0 {
                Unit::new_normalize(direction)
            } else {
                continue;
            };
            player_transform.append_translation_along(direction, player.speed);
        }
    }
}

pub struct PlayerHealthSystem;
impl<'a> System<'a> for PlayerHealthSystem {
    type SystemData = (WriteStorage<'a, components::Player>);

    fn run(&mut self, mut players: Self::SystemData) {
        for player in (&mut players).join() {
            player.health += 1;
        }
    }
}

pub struct PlayerControlSystem;
impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        ReadStorage<'a, components::Player>,
        ReadStorage<'a, components::markers::Controllable>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (players, controllables, mut transforms, input): Self::SystemData) {
        for (player, _, transform) in (&players, &controllables, &mut transforms).join() {
            let direction = Vector3::new(
                -input.axis_value("horizontal").unwrap(),
                input.axis_value("vertical").unwrap(),
                0.0,
            );
            let direction = if direction.magnitude() > 0.1 {
                Unit::new_normalize(direction)
            } else {
                continue;
            };
            transform.append_translation_along(direction, player.speed);
        }
    }
}

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
