use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::{AnimatedTile, TileBundle, TilePos, TileStorage, TileTexture};
use bevy_ecs_tilemap::{TilemapBundle, TilemapPlugin};
use rand::seq::IteratorRandom;
use rand::thread_rng;

mod helpers;

struct TilemapMetadata {
    size: TilemapSize,
    tile_size: TilemapTileSize,
    grid_size: TilemapGridSize,
}

const BACKGROUND: &'static str = "tiles.png";
const BACKGROUND_METADATA: TilemapMetadata = TilemapMetadata {
    size: TilemapSize { x: 20, y: 20 },
    tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
    grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
};

const FLOWERS: &'static str = "flower_sheet.png";
const FLOWERS_METADATA: TilemapMetadata = TilemapMetadata {
    size: TilemapSize { x: 10, y: 10 },
    tile_size: TilemapTileSize { x: 32.0, y: 32.0 },
    grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
};

fn create_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load(BACKGROUND);

    let tilemap_entity = commands.spawn().id();

    let TilemapMetadata {
        size,
        grid_size,
        tile_size,
    } = BACKGROUND_METADATA;

    let mut tilemap_storage = TileStorage::empty(size);

    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            // Here we let the tile storage component know what tiles we have.
            tilemap_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            size,
            grid_size,
            tile_size,
            storage: tilemap_storage,
            texture: TilemapTexture(texture_handle.clone()),
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(&size, &tile_size, 0.0),
            ..Default::default()
        });
}

fn create_animated_flowers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load(FLOWERS);

    let TilemapMetadata {
        size,
        grid_size,
        tile_size,
    } = FLOWERS_METADATA;

    let mut tilemap_storage = TileStorage::empty(size);

    let tilemap_entity = commands.spawn().id();

    // Choose 10 random tiles to contain flowers.
    let mut rng = thread_rng();
    let mut indices: Vec<(u32, u32)> = Vec::with_capacity((size.x * size.y) as usize);
    for x in 0..size.x {
        for y in 0..size.y {
            indices.push((x, y));
        }
    }
    for (x, y) in indices.into_iter().choose_multiple(&mut rng, 10) {
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
            .spawn()
            .insert_bundle(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture: TileTexture(0),
                ..Default::default()
            })
            .id();
        tilemap_storage.set(&tile_pos, Some(tile_entity));
        // To enable animation, we must insert the `AnimatedTile` component on
        // each tile that is to be animated.
        commands.entity(tile_entity).insert(AnimatedTile {
            start: 0,
            end: 13,
            speed: 0.95,
        });
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size,
            storage: tilemap_storage,
            texture: TilemapTexture(texture_handle.clone()),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(&size, &tile_size, 1.0),
            ..Default::default()
        });
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Animated Map Example"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_startup_system(create_background)
        .add_startup_system(create_animated_flowers)
        .add_system(helpers::camera::movement)
        .run();
}