//!
//! Renders actors in the game world.
//!
//! Actors are movable movable entities
//! and are expected to be updated every
//! frame.
//!

use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::ActiveCamera,
};

use crate::components::markers::Actor;
use crate::components::Position;
use crate::systems::render::common::TILE_SIZE;

pub struct ActorRenderSystem;

impl<'a> System<'a> for ActorRenderSystem {
    type SystemData = (
        Read<'a, ActiveCamera>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (camera, actors, positions, mut transforms): Self::SystemData) {
        let (cam_pos, cam_trans) =
            match camera.entity.map(|e| (positions.get(e), transforms.get(e))) {
                Some((Some(p), Some(t))) => (p, t.translation().clone()),
                _ => return,
            };

        for (_, pos, trans) in (&actors, &positions, &mut transforms).join() {
            let mut t = cam_trans + (pos.trans - cam_pos.trans) * TILE_SIZE;
            t.z = 0.01;
            log::debug!("actor trans: {} {} {}", t.x, t.y, t.z);
            trans.set_translation(t);
        }
    }
}
