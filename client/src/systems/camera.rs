use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Read, System, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::prelude::World;
use amethyst::renderer::ActiveCamera;
use shrev::{EventChannel, ReaderId};

use crate::components::Position;
use crate::events::WorldEvent;
use crate::systems::render::common::TILE_SIZE;
use crate::systems::util::{CameraTarget, Target};

/// Have the active camera follow a target.
pub struct CameraControlSystem {
    pub zoom: f32,
    reader_id: ReaderId<WorldEvent>,
}

impl CameraControlSystem {
    pub fn new(world: &mut World, zoom: f32) -> Self {
        let reader_id = world
            .fetch_mut::<EventChannel<WorldEvent>>()
            .register_reader();

        Self { reader_id, zoom }
    }
}

impl<'a> System<'a> for CameraControlSystem {
    type SystemData = (
        Read<'a, ActiveCamera>,
        Read<'a, CameraTarget>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        Read<'a, InputHandler<StringBindings>>,
        Write<'a, EventChannel<WorldEvent>>,
    );

    fn run(
        &mut self,
        (camera, target, mut positions, mut transforms, time, input, mut dispatch): Self::SystemData,
    ) {
        let target_pos = match target.target {
            Target::Position(p) => Some(p.trans.clone()),
            Target::Entity(e, offset) => positions.get(e).map(|p| {
                let mut trans = p.trans;
                trans.z += offset;
                trans
            }),
            Target::None => None,
        };

        let (cam_pos, cam_trans) = {
            let cam = match camera.entity {
                Some(c) => c,
                None => return,
            };
            match (positions.get_mut(cam), transforms.get_mut(cam)) {
                (Some(p), Some(t)) => (p, t),
                _ => return,
            }
        };

        self.handle_zoom(time.delta_real_seconds(), input, cam_trans);
        self.handle_generation(&mut dispatch, cam_pos);
        <CameraControlSystem>::handle_camera_movement(
            time.delta_real_seconds(),
            target_pos,
            cam_pos,
            cam_trans,
        )
    }
}

impl CameraControlSystem {
    fn handle_zoom(
        &mut self,
        time: f32,
        input: Read<InputHandler<StringBindings>>,
        cam_trans: &mut Transform,
    ) {
        self.zoom += input.axis_value("zoom").unwrap() * time;
        let scale_factor = 1.0 / self.zoom;
        cam_trans.set_scale(Vector3::new(scale_factor, scale_factor, 1.0));
    }

    fn handle_generation(
        &mut self,
        dispatch: &mut Write<EventChannel<WorldEvent>>,
        cam_pos: &mut Position,
    ) {
        let should_update = dispatch.read(&mut self.reader_id).any(|e| match e {
            WorldEvent::GenerateRequest => true,
            _ => false,
        });
        if should_update {
            dispatch.single_write(WorldEvent::Generate(cam_pos.trans.x, cam_pos.trans.y));
            dispatch.single_write(WorldEvent::Updated);
        }
    }

    fn handle_camera_movement(
        time: f32,
        target_pos: Option<Vector3<f32>>,
        cam_pos: &mut Position,
        cam_trans: &mut Transform,
    ) {
        if let Some(target_pos) = target_pos {
            let dir = target_pos - cam_pos.trans;
            log::debug!(
                "camera: {} {} {}, target: {} {} {}, diff {} {} {}",
                cam_pos.trans.x,
                cam_pos.trans.y,
                cam_pos.trans.z,
                target_pos.x,
                target_pos.y,
                target_pos.z,
                dir.x,
                dir.y,
                dir.z,
            );

            cam_pos.trans += dir * time;
            cam_trans.append_translation(dir * time * TILE_SIZE);
        }
    }
}
