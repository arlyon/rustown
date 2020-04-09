use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use amethyst::{
    core::math::Vector3,
    ecs::{Entities, Join, Read, System, Write, WriteStorage},
    prelude::World,
};
use noise::{NoiseFn, OpenSimplex, Seedable};
use shrev::{EventChannel, ReaderId};

use crate::components::{Position, Terrain, TerrainVariant};
use crate::config;
use crate::events::WorldEvent;

pub struct WorldGenerationSystem {
    pub noise: OpenSimplex,
    reader_id: ReaderId<WorldEvent>,
}

impl WorldGenerationSystem {
    pub fn new(world: &mut World, seed: String) -> Self {
        let noise = noise::OpenSimplex::default();
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        noise.set_seed(hasher.finish() as u32);

        let reader_id = world
            .fetch_mut::<EventChannel<WorldEvent>>()
            .register_reader();

        WorldGenerationSystem { noise, reader_id }
    }
}

impl<'a> System<'a> for WorldGenerationSystem {
    type SystemData = (
        Read<'a, config::Settings>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Terrain>,
        Write<'a, EventChannel<WorldEvent>>,
    );

    fn run(&mut self, (settings, entities, mut positions, mut terrain, events): Self::SystemData) {
        // todo(arlyon) handle multiple generate requests
        let generate_request = events
            .read(&mut self.reader_id)
            .filter_map(|e| match e {
                WorldEvent::Generate(x, y) => Some((*x, *y)),
                _ => None,
            })
            .last();

        if let Some((x, y)) = generate_request {
            log::debug!("Generating terrain");

            for _ in terrain.count()..(settings.render_distance * 2).pow(2) as usize {
                let e = entities.create();
                terrain.insert(e, Terrain::default()).unwrap();
                positions.insert(e, Position::default()).unwrap();
            }

            let x_min = x.floor() as i32 - settings.render_distance as i32;
            let x_max = x.floor() as i32 + settings.render_distance as i32;
            let y_min = y.floor() as i32 - settings.render_distance as i32;

            log::debug!(
                "generating x: {} x_min: {}, y: {} y_min: {}",
                x,
                x_min,
                y,
                y_min
            );

            let mut x = x_min;
            let mut y = y_min;
            for (p, t) in (&mut positions, &mut terrain).join() {
                p.trans = Vector3::new(x as f32, y as f32, 0.0);
                t.variant = match self.noise.get([x as f64 / 10.0, y as f64 / 10.0]) {
                    x if x < -0.15 => TerrainVariant::Water,
                    x if x < -0.05 => TerrainVariant::Dirt,
                    _ => TerrainVariant::Grass,
                };

                x += 1;
                if x == x_max {
                    y += 1;
                    x = x_min;
                };
            }
        }
    }
}
