use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bluenoise::BlueNoise;
use rand::{prelude::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;

use super::{
    animation::{AnimationSet, AnimationState},
    map::TileSize,
    sound::Sound,
};

#[derive(Component)]
pub struct CoinBag(u32);

#[derive(Component)]
pub struct EvilAI;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Position(Transform);

#[derive(Component)]
pub struct Controllable;

#[derive(Component)]
pub struct MovementSpeed(f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Following(Entity);

pub fn register_systems(app: &mut App) {
    app.add_startup_system(setup_king.system().label("game"))
        .add_startup_system(setup_targets.system().after("sound"))
        .add_system(move_controllables.system())
        .add_system(evil_ai.system())
        .add_system(velocity.system())
        .add_system(follow.system())
        .add_system(coin_bag.system());
}

/// Follow the camera to the player.
fn follow(query: Query<(Entity, &Following)>, mut transforms: Query<&mut Transform>) {
    for (e, f) in query.iter() {
        let followed = { transforms.get_mut(f.0).unwrap().translation };
        let mut follower = transforms.get_mut(e).unwrap();
        follower.translation = followed;
    }
}

fn setup_targets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    tile_size: Res<TileSize>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    debug!("Spawning objectives");

    let mut rng = rand::thread_rng();
    let noise = BlueNoise::<Pcg64Mcg>::new(9.0, 9.0, 1.0);

    let coin_handle = {
        let asset = assets.load("coin_gold.png");
        let texture_atlas = TextureAtlas::from_grid(asset, Vec2::new(32.0, 32.0), 8, 1);
        texture_atlases.add(texture_atlas)
    };

    let skelly_idle = {
        let king = assets.load("entity/Skeleton Idle.png");
        let king_atlas = TextureAtlas::from_grid(king, Vec2::new(24.0, 32.0), 11, 1);
        texture_atlases.add(king_atlas)
    };

    let skelly_walk = {
        let king = assets.load("entity/Skeleton Walk.png");
        let king_atlas = TextureAtlas::from_grid(king, Vec2::new(22.0, 33.0), 13, 1);
        texture_atlases.add(king_atlas)
    };

    let skelly_attack = {
        let king = assets.load("entity/Skeleton Attack.png");
        let king_atlas = TextureAtlas::from_grid(king, Vec2::new(43.0, 37.0), 18, 1);
        texture_atlases.add(king_atlas)
    };

    let sound = Sound {
        cash: ["sound/cash.ogg", "sound/cash2.ogg"]
            .into_iter()
            .map(|s| assets.load(s))
            .collect(),
        attack: [
            "sound/skeleton/MON_OrcSkeleton_Attack01.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack02.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack03.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack04.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack05.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack06.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack07.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack08.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack09.ogg",
            "sound/skeleton/MON_OrcSkeleton_Attack10.ogg",
        ]
        .into_iter()
        .map(|s| assets.load(s))
        .collect(),
        walk: [
            "sound/skeleton/FX_Bone_Movement_Aggressive07.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive08.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive09.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive10.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive11.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive12.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive13.ogg",
            "sound/skeleton/FX_Bone_Movement_Aggressive14.ogg",
        ]
        .into_iter()
        .map(|s| assets.load(s))
        .collect(),
    };

    commands.insert_resource(sound.clone());

    let x = texture_atlases.get(coin_handle.clone()).unwrap();
    for point in noise.take(10000) {
        let position = Vec2::new(point.x as f32, -(point.y as f32));
        let translation = (position * tile_size.0).extend(0.0);
        let transform = Transform {
            translation,
            ..Default::default()
        };

        if rng.gen() {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: coin_handle.clone(),
                    transform,
                    sprite: TextureAtlasSprite {
                        index: rng.gen::<usize>() % x.len(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.15, true))
                .insert(Item);
        } else {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: skelly_idle.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.1, true))
                .insert(EvilAI)
                .insert(MovementSpeed(50.0))
                .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
                .insert(AnimationState::Standard)
                .insert(sound.clone())
                .insert(AnimationSet {
                    idle: skelly_idle.clone(),
                    up: skelly_walk.clone(),
                    down: skelly_walk.clone(),
                    left: skelly_walk.clone(),
                    right: skelly_walk.clone(),
                    attack: skelly_attack.clone(),
                });
        }
    }
}

fn setup_king(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    debug!("Setting up game");

    let king_idle = {
        let king = assets.load("entity/spr_KingIdle_strip_no_bkg.png");
        let king_atlas = TextureAtlas::from_grid(king, Vec2::new(128.0, 128.0), 18, 1);
        texture_atlases.add(king_atlas)
    };

    let king_walk = {
        let king = assets.load("entity/spr_KingWalk_strip_no_bkg.png");
        let king_atlas = TextureAtlas::from_grid(king, Vec2::new(128.0, 128.0), 8, 1);
        texture_atlases.add(king_atlas)
    };

    let king = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: king_idle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(MovementSpeed(100.0))
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert(Controllable)
        .insert(CoinBag(0))
        .insert(AnimationSet {
            idle: king_idle,
            up: king_walk.clone(),
            down: king_walk.clone(),
            left: king_walk.clone(),
            right: king_walk.clone(),
            attack: king_walk,
        })
        .id();

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 1000.0,
        ))))
        .insert(Following(king));

    debug!("Game set up");
}

fn move_controllables(
    mut query: Query<(&Controllable, &MovementSpeed, &mut Velocity)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut direction = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    for (_, speed, mut velocity) in query.iter_mut() {
        velocity.0 = direction.extend(0.0).clamp_length_max(1.0) * speed.0;
    }
}

fn evil_ai(
    mut enemies: Query<
        (
            Entity,
            &mut Velocity,
            &mut MovementSpeed,
            &mut AnimationState,
        ),
        With<EvilAI>,
    >,
    mut queries: QuerySet<(
        QueryState<&Transform, With<Controllable>>,
        QueryState<&Transform>,
    )>,
) {
    'outer: for (e, mut v, m, mut anim) in enemies.iter_mut() {
        let curr = queries.q1().get(e).unwrap().translation;
        for transform in queries.q0().iter() {
            let distance_sqr = curr.distance_squared(transform.translation);
            if distance_sqr < 500.0 {
                *anim = AnimationState::Combat;
            } else {
                *anim = AnimationState::Standard;
                if distance_sqr < 10_000.0 {
                    let direction = (transform.translation - curr).normalize();

                    v.0 = direction * m.0;
                    v.0.z = 0.0;
                    continue 'outer;
                }
            }
        }
        v.0 = Vec3::new(0.0, 0.0, 0.0);
    }
}

fn velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn coin_bag(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CoinBag)>,
    sound: Res<Sound>,
    audio: Res<Audio>,
    mut locations: QuerySet<(
        QueryState<&Transform>,
        QueryState<(Entity, &Transform), With<Item>>,
    )>,
) {
    let mut rng = rand::thread_rng();

    // for each entity with a coin bag
    for (e, mut cb) in query.iter_mut() {
        let q0 = locations.q0();
        let loc = q0.get(e).unwrap().translation;
        let mut picked_coin = Option::None;
        for (e, location) in locations.q1().iter() {
            let distance_sqr = loc.distance_squared(location.translation);
            if distance_sqr < 200.0 {
                cb.0 += 1;
                debug!("collected coin");
                picked_coin.replace(e);
                break;
            }
        }
        if let Some(e) = picked_coin {
            commands.entity(e).despawn();
            audio.play(sound.cash.choose(&mut rng).unwrap().clone());
        }
    }
}
