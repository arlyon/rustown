use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

enum Tile {
    Wall,
    Floor,
    Entry,
    Exit,
}

pub fn register_systems(app: &mut App) {
    app.insert_resource(TileSize(Vec2::splat(64.0)))
        .add_startup_system(load_map.system().label("map"));
}

#[derive(Clone, Copy)]
pub struct TileSize(pub Vec2);

fn load_map(mut commands: Commands, assets: Res<AssetServer>, tile_size: Res<TileSize>) {
    debug!("Loading map");

    let floor_handle = assets.load("floor.png");
    let wall_handle = assets.load("decor/bush/Bushes32x32_07.png");

    let map_path = "assets/map.txt";
    let f = File::open(map_path).unwrap();
    for (i, j, c) in BufReader::new(f).lines().enumerate().flat_map(|(i, l)| {
        l.unwrap()
            .chars()
            .enumerate()
            .map(|(j, c)| (i, j, c))
            .collect::<Vec<_>>()
    }) {
        let tile = match c {
            'x' => Tile::Wall,
            '.' => Tile::Entry,
            ',' => Tile::Exit,
            _ => Tile::Floor,
        };

        let position = Vec2::new(j as f32, -(i as f32));
        let translation = (position * tile_size.0).extend(0.0);
        let transform = Transform {
            translation,
            ..Default::default()
        };

        if let Tile::Wall = tile {
            commands.spawn().insert_bundle(SpriteBundle {
                texture: wall_handle.clone(),
                transform,
                sprite: Sprite {
                    custom_size: Some(tile_size.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        commands.spawn().insert_bundle(SpriteBundle {
            texture: floor_handle.clone(),
            transform: transform.with_translation(translation + Vec3::new(0.0, 0.0, -0.1)),
            sprite: Sprite {
                custom_size: Some(tile_size.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
