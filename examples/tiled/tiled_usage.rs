use crate::tiled::*;
use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_ecs_tilemap::prelude::*;

#[path = "../helpers/mod.rs"]
mod helpers;
mod tiled;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let handle: Handle<TiledMap> = asset_server.load("level_1/level.tmx");
    //let handle: Handle<TiledMap> = asset_server.load("map.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(-360.0, -224.0, 0.0)
        .with_scale(Vec3::ONE * 2.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 720.0,
            height: 448.0,
            title: String::from("Tiled map editor example"),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::parallax_camera::movement)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .run();
}
