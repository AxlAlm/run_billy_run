use bevy::prelude::*;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_ecs_tilemap::prelude::*;

use crate::components::Lava;
use crate::{GameTextures, WinSize, TILE_SIZE};

pub struct LavaPlugin;

impl Plugin for LavaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            //.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(TilemapPlugin)
            .add_startup_system(startup);
    }
}

#[derive(Default, Component)]
struct LastUpdate {
    value: f64,
}

// fn spawn_lava(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>) {
//     let bottom = -win_size.height / 2.;
//     commands
//         .spawn_bundle(SpriteBundle {
//             texture: game_textures.lava.clone(),
//             transform: Transform {
//                 translation: Vec3::new(0.0, bottom + 10.0, 0.0), //
//                 scale: Vec3::new(0.1, 0.1, 0.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(Lava);
// }

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("lava.png");

    let tilemap_size = TilemapSize { x: 10, y: 10 };

    let mut tile_storage = TileStorage::empty(tilemap_size);

    let tile_size2: f32 = 128.0;

    let tile_size = TilemapTileSize {
        x: tile_size2,
        y: tile_size2,
    };

    // For the purposes of this example, we consider a square tile map,
    // // where diagonals are also considered to be neighbors.
    // let tilemap_type = TilemapType::Square {
    //     neighbors_include_diagonals: true,
    // };

    // Create a tilemap entity a little early
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    let tilemap_entity = commands.spawn().id();

    // Spawn a 32 by 32 tilemap.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
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
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize {
                x: tile_size2,
                y: tile_size2,
            },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });
}
