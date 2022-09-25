use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize},
    tiles::{TileBundle, TilePos, TileStorage, TileTexture},
    TilemapBundle,
};
use std::collections::HashMap;

use bevy::reflect::TypeUuid;
use bevy::{
    asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
};

use ldtk_rust::{Project, TilesetDefinition};

#[derive(Default)]
pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<LdtkMap>()
            .add_asset_loader(LdtkLoader)
            .add_system(process_loaded_tile_maps);
    }
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct LdtkMap {
    pub project: ldtk_rust::Project,
    pub tilesets: HashMap<i64, Handle<Image>>,
}

#[derive(Default, Component)]
pub struct LdtkMapConfig {
    pub selected_level: usize,
}

#[derive(Default, Bundle)]
pub struct LdtkMapBundle {
    pub ldtk_map: Handle<LdtkMap>,
    pub ldtk_map_config: LdtkMapConfig,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub struct Tileset<'a> {
    id: i64,
    path: AssetPath<'a>,
    data: Handle<Image>,
}

pub struct LdtkLoader;

// Result<_,sarde_json::Error>
// Result<Project, String>
// Result<Project, Error>
fn load_ldtk_project(bytes: &[u8]) -> Result<Project, serde_json::Error> {
    let project: Project = serde_json::from_slice(bytes)?;
    Ok(project)
}

fn _load_tileset(tileset: &TilesetDefinition, load_context: &LoadContext) -> Tileset{

    let path: AssetPath = load_context.path().parent().unwrap().join(tileset.rel_path).into();
    let data : Handle<Image> =  load_context.get_handle(x.1.clone());
    let tileset = Tileset {
                id :tileset.uid,
                path:path,
                data: data
    };

    tileset

};

fn _get_tilesets<'a>(project: &Project, load_context: &LoadContext) -> Vec<Tileset<'a>> {
    // set dependecies
    let tilesets = Vec::new();

    for tileset in project.defs.tilesets.iter() {

        if let Some(tileset) = &tileset.rel_path {
            continue;
        }

        tilesets.push(_load_tileset(&tileset, load_context));
    }
    tilesets
}

fn get_tileset_paths<'a>(
    project: &Project,
    load_context: &LoadContext,
) -> Vec<(i64, AssetPath<'a>)> {
    // set dependecies
    return project
        .defs
        .tilesets
        .iter()
        .filter_map(|tileset| {
            if let Some(rel_path) = &tileset.rel_path {
                Some((
                    tileset.uid,
                    load_context.path().parent().unwrap().join(rel_path).into(),
                ))
            } else {
                None
            }
        })
        .collect();
}

fn load_tilesets(
    tilesets: &Vec<(i64, AssetPath)>,
    load_context: &LoadContext,
) -> HashMap<i64, Handle<Image>> {
    return tilesets
        .iter()
        .map(|x| (x.0, load_context.get_handle(x.1.clone())))
        .collect();
}

impl AssetLoader for LdtkLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {



            // laod project
            let project = load_ldtk_project(bytes)?;
            let tilesets_paths = get_tileset_paths(&project, &load_context);
            let tilesets = load_tilesets(&tilesets_paths, &load_context);

            println!("{:?}", tilesets_paths);
            println!("{:?}", tilesets);

            let ldtk_map: LdtkMap = LdtkMap {
                project,
                tilesets: tilesets,
            };

            let loaded_asset = LoadedAsset::new(ldtk_map);

            load_context.set_default_asset(
                loaded_asset
                    .with_dependencies(tilesets_paths.iter().map(|x| x.1.clone()).collect()),
            );

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["ldtk"];
        EXTENSIONS
    }
}

pub fn process_loaded_tile_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<LdtkMap>>,
    maps: Res<Assets<LdtkMap>>,
    mut query: Query<(Entity, &Handle<LdtkMap>, &LdtkMapConfig)>,
    new_maps: Query<&Handle<LdtkMap>, Added<Handle<LdtkMap>>>,
) {
    let mut changed_maps = Vec::<Handle<LdtkMap>>::default();

    for event in map_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                changed_maps.push(handle.clone());
            }

            AssetEvent::Modified { handle } => {
                changed_maps.push(handle.clone());
            }

            AssetEvent::Removed { handle } => {
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps = changed_maps
                    .into_iter()
                    .filter(|changed_handle| changed_handle == handle)
                    .collect();
            }
        }
    }

    // If we have new map entities, add them to the changed_maps list
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.clone());
    }

    for changed_map in changed_maps.iter() {
        for (entity, map_handle, map_config) in query.iter_mut() {
            // only deal with currently changed map
            if map_handle != changed_map {
                continue;
            }
            if let Some(ldtk_map) = maps.get(map_handle) {
                // Despawn all existing tilemaps for this LdtkMap
                commands.entity(entity).despawn_descendants();

                // Pull out tilesets and their definitions into a new hashmap
                let mut tilesets = HashMap::new();
                ldtk_map.project.defs.tilesets.iter().for_each(|tileset| {
                    tilesets.insert(
                        tileset.uid,
                        (
                            ldtk_map.tilesets.get(&tileset.uid).unwrap().clone(),
                            tileset.clone(),
                        ),
                    );
                });

                let default_grid_size = ldtk_map.project.default_grid_size;
                let level = &ldtk_map.project.levels[map_config.selected_level];

                let map_tile_count_x = (level.px_wid / default_grid_size) as u32;
                let map_tile_count_y = (level.px_hei / default_grid_size) as u32;

                let size = TilemapSize {
                    x: map_tile_count_x,
                    y: map_tile_count_y,
                };

                // We will create a tilemap for each layer in the following loop
                for (layer_id, layer) in level
                    .layer_instances
                    .as_ref()
                    .unwrap()
                    .iter()
                    .rev()
                    .enumerate()
                {
                    if let Some(uid) = layer.tileset_def_uid {
                        let (texture, tileset) = tilesets.get(&uid).unwrap().clone();

                        // Tileset-specific tilemap settings
                        let tile_size = TilemapTileSize {
                            x: tileset.tile_grid_size as f32,
                            y: tileset.tile_grid_size as f32,
                        };

                        // Pre-emptively create a map entity for tile creation
                        let map_entity = commands.spawn().id();

                        // Create tiles for this layer from LDtk's grid_tiles and auto_layer_tiles
                        let mut storage = TileStorage::empty(size);

                        for tile in layer.grid_tiles.iter().chain(layer.auto_layer_tiles.iter()) {
                            let mut position = TilePos {
                                x: (tile.px[0] / default_grid_size) as u32,
                                y: (tile.px[1] / default_grid_size) as u32,
                            };

                            position.y = map_tile_count_y - position.y - 1;

                            let tile_entity = commands
                                .spawn()
                                .insert_bundle(TileBundle {
                                    position,
                                    tilemap_id: TilemapId(map_entity),
                                    texture: TileTexture(tile.t as u32),
                                    ..default()
                                })
                                .id();

                            storage.set(&position, Some(tile_entity));
                        }

                        // Create the tilemap
                        commands.entity(map_entity).insert_bundle(TilemapBundle {
                            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
                            size,
                            storage,
                            texture: TilemapTexture(texture),
                            tile_size,
                            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                                &size,
                                &tile_size,
                                layer_id as f32,
                            ),
                            ..default()
                        });
                    }
                }
            }
        }
    }
}
